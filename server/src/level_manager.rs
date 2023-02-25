use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use common::{VResult, Vector3f};
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use level::ChunkManager;
use p384::elliptic_curve::rand_core::le;
use parking_lot::{RwLock, RwLockReadGuard};
use tokio::sync::oneshot::Receiver;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::command::Command;
use crate::config::SERVER_CONFIG;
use crate::entity::player::{
    player_movement, ActiveEntity, Player, PlayerBundle, PlayerMoveLabel,
    Transform, broadcast_event_handler,
};
use crate::network::Skin;
use crate::network::packets::{GameMode, login};
use crate::network::packets::login::{PermissionLevel, Login};
use crate::network::session::Session;
use crate::network::{
    packets::{GameRule, GameRulesChanged},
    session::SessionManager,
};

use bevy_ecs::prelude::*;

/// Interval between standard Minecraft ticks.
const LEVEL_TICK_INTERVAL: Duration = Duration::from_millis(1000 / 20);
static RUNTIME_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct LevelManager {
    /// Used to load world data from disk.
    chunks: Arc<ChunkManager>,
    /// List of commands available in this level.
    commands: DashMap<String, Command>,
    /// Currently set game rules.
    game_rules: DashMap<String, GameRule>,
    /// Used to broadcast level events to the sessions.
    session_manager: Arc<SessionManager>,
    /// Current world tick.
    /// This is the standard Minecraft tick.
    /// The level is ticked 20 times every second.
    tick: AtomicU64,
    token: CancellationToken,

    schedule: RwLock<Schedule>,
    ecs_world: RwLock<World>,
}

impl LevelManager {
    pub fn new(
        session_manager: Arc<SessionManager>,
        token: CancellationToken,
    ) -> VResult<(Arc<Self>, Receiver<()>)> {
        let (world_path, autosave_interval) = {
            let config = SERVER_CONFIG.read();
            (config.level_path.clone(), config.autosave_interval)
        };

        let (chunks, chunk_notifier) =
            ChunkManager::new(world_path, autosave_interval, token.clone())?;

        let mut schedule = Schedule::default();
        schedule.add_stage(
            PlayerMoveLabel,
            SystemStage::parallel()
                .with_system(player_movement)
                .with_system(broadcast_event_handler)
        );

        let manager = Arc::new(Self {
            chunks,
            commands: DashMap::new(),
            game_rules: DashMap::from_iter([
                (
                    "showcoordinates".to_owned(),
                    GameRule::ShowCoordinates(false),
                ),
                (
                    "naturalregeneration".to_owned(),
                    GameRule::NaturalRegeneration(false),
                ),
            ]),
            session_manager,
            tick: AtomicU64::new(0),
            token,
            ecs_world: RwLock::new(World::new()),
            schedule: RwLock::new(schedule),
        });

        let clone = manager.clone();
        tokio::spawn(async move {
            clone.level_ticker_job().await;
        });

        Ok((manager, chunk_notifier))
    }

    pub fn add_player(&self, session: &Arc<Session>, login_data: Login) -> Entity {
        self.ecs_world.write().spawn(PlayerBundle {
            entity: ActiveEntity {
                runtime_id: RUNTIME_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
            },
            player: Player {
                username: login_data.identity.display_name,
                xuid: login_data.identity.xuid,
                uuid: login_data.identity.uuid,
                game_mode: GameMode::Creative,
                permission_level: PermissionLevel::Member,
                skin: login_data.skin,
                session: Arc::clone(session),
                device_os: login_data.user_data.build_platform
            },
            transform: Transform {
                position: Vector3f::zero(),
                rotation: Vector3f::zero(),
            },
        }).id()
    }
    
    pub fn remove_player(&self, entity: Entity) {
        self.ecs_world.write().despawn(entity);
    }

    /// Returns the requested command
    #[inline]
    pub fn get_command(&self, name: &str) -> Option<Ref<String, Command>> {
        self.commands.get(name)
    }

    /// Returns a list of available commands.
    #[inline]
    pub const fn get_commands(&self) -> &DashMap<String, Command> {
        &self.commands
    }

    /// Adds a command to the list of available commands.
    #[inline]
    pub fn add_command(&self, command: Command) {
        self.commands.insert(command.name.clone(), command);
    }

    #[inline]
    pub fn add_many_commands(&self, commands: &[Command]) {
        commands.iter().for_each(|cmd| {
            self.commands.insert(cmd.name.clone(), cmd.clone());
        });
    }

    /// Returns the specified game rule
    #[inline]
    pub fn get_game_rule(&self, name: &str) -> Option<GameRule> {
        self.game_rules.get(name).map(|kv| *kv.value())
    }

    /// Returns a list of currently applied game rules.
    #[inline]
    pub fn get_game_rules(&self) -> Vec<GameRule> {
        self.game_rules
            .iter()
            .map(|kv| *kv.value())
            .collect::<Vec<_>>()
    }

    /// Sets the value of a game rule, returning the old value if there was one.
    #[inline]
    pub fn set_game_rule(&self, game_rule: GameRule) -> Option<GameRule> {
        let name = game_rule.name();

        self.session_manager
            .broadcast(GameRulesChanged { game_rules: &[game_rule] });
        self.game_rules.insert(name.to_owned(), game_rule)
    }

    /// Modifies multiple game rules at the same time.
    /// This function also notifies all the clients of the change.
    #[inline]
    pub fn set_game_rules(&self, game_rules: &[GameRule]) {
        for game_rule in game_rules {
            let name = game_rule.name();
            self.game_rules.insert(name.to_owned(), *game_rule);
        }

        self.session_manager
            .broadcast(GameRulesChanged { game_rules });
    }

    async fn level_ticker_job(self: Arc<Self>) {
        let mut interval = tokio::time::interval(LEVEL_TICK_INTERVAL);
        while !self.token.is_cancelled() {
            self.schedule.write().run(&mut self.ecs_world.write());
            interval.tick().await;
        }
    }
}
