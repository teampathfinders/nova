use std::net::SocketAddr;
use std::num::NonZeroU64;
use std::sync::atomic::{
    AtomicBool, AtomicU16, AtomicU32, AtomicU64, Ordering,
};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use aes::cipher::typenum::NonZero;
use bevy_ecs::prelude::Entity;
use bytes::{Bytes, BytesMut};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, mpsc, OnceCell};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::crypto::{Encryptor, IdentityData, UserData};
use crate::instance_manager::InstanceManager;
use crate::level_manager::LevelManager;
use crate::network::packets::login::{DeviceOS, Disconnect, PermissionLevel};
use crate::network::packets::{
    ConnectedPacket, GameMode, MessageType, Packet, PlayerListRemove,
    TextMessage,
};
use crate::network::raknet::{BroadcastPacket, RaknetData};
use crate::network::Skin;
use common::{bail, Serialize, Vector3f};
use common::{error, VResult};

use super::SessionManager;

/// Sessions directly correspond to clients connected to the server.
///
/// Anything that has to do with specific clients must be communicated with their associated sessions.
/// The server does not interact with clients directly, everything is done through these sessions.
#[derive(Debug)]
pub struct Session {
    /// Used to encrypt and decrypt packets.
    pub encryptor: OnceCell<Encryptor>,
    /// Whether the client supports the chunk cache.
    pub cache_support: OnceCell<bool>,
    /// Whether the client has fully been initialised.
    /// This is set to true after receiving the [`SetLocalPlayerAsInitialized`](crate::network::packets::SetLocalPlayerAsInitialized) packet
    pub initialized: AtomicBool,
    /// Manages entire world.
    pub level_manager: Arc<LevelManager>,
    /// Sends packets into the broadcasting channel.
    pub broadcast: broadcast::Sender<BroadcastPacket>,

    /// Indicates whether this session is active.
    pub active: CancellationToken,

    /// Current tick of this session, this is increased every [`TICK_INTERVAL`].
    pub current_tick: AtomicU64,
    /// Raknet-specific data.
    pub raknet: RaknetData,
    pub entity: OnceCell<Entity>
}

impl Session {
    /// Creates a new session.
    pub fn new(
        broadcast: broadcast::Sender<BroadcastPacket>,
        mut receiver: mpsc::Receiver<Bytes>,
        level_manager: Arc<LevelManager>,
        ipv4_socket: Arc<UdpSocket>,
        address: SocketAddr,
        mtu: u16,
        guid: u64,
    ) -> Arc<Self> {
        let session = Arc::new(Self {
            encryptor: OnceCell::new(),
            cache_support: OnceCell::new(),
            initialized: AtomicBool::new(false),
            broadcast,
            level_manager,

            active: CancellationToken::new(),

            current_tick: AtomicU64::new(0),
            entity: OnceCell::new(),
            raknet: RaknetData {
                udp_socket: ipv4_socket,
                mtu,
                guid,
                last_update: RwLock::new(Instant::now()),
                batch_sequence_number: Default::default(),
                sequence_index: Default::default(),
                ack_index: Default::default(),
                compound_id: Default::default(),
                client_batch_number: Default::default(),
                compound_collector: Default::default(),
                order_channels: Default::default(),
                send_queue: Default::default(),
                confirmed_packets: Mutex::new(Vec::new()),
                compression_enabled: AtomicBool::new(false),
                address,
                recovery_queue: Default::default(),
            },
        });

        // Start processing jobs.
        // These jobs run in separate tasks, therefore the session has to be cloned.
        session.clone().start_ticker_job();
        session.clone().start_packet_job(receiver);
        session
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn get_encryptor(&self) -> VResult<&Encryptor> {
        self.encryptor.get().ok_or_else(|| {
            error!(NotInitialized, "Encryption has not been initialised yet")
        })
    }

    /// Returns the randomly generated GUID given by the client itself.
    #[inline]
    pub const fn get_guid(&self) -> u64 {
        self.raknet.guid
    }

    /// Kicks the session from the server, displaying the given menu.
    pub fn kick<S: AsRef<str>>(&self, message: S) -> VResult<()> {
        let disconnect_packet = Disconnect {
            message: message.as_ref(),
            hide_message: false,
        };
        self.send(disconnect_packet)?;
        // self.flag_for_close();
        // FIXME: Client sends disconnect and acknowledgement packet after closing.

        Ok(())
    }

    /// Returns whether the session is currently active.
    ///
    /// If this returns false, any remaining associated processes should be stopped as soon as possible.
    #[inline]
    pub fn is_active(&self) -> bool {
        !self.active.is_cancelled()
    }

    #[inline]
    pub async fn cancelled(&self) {
        self.active.cancelled().await;
    }
}
