use std::{
    sync::{atomic::Ordering, Arc},
    time::{Duration, Instant},
};
use tokio::sync::mpsc;

use bytes::Bytes;
use common::VResult;

use crate::network::{
    packets::{MessageType, PlayerListRemove, TextMessage},
    session::Session,
};

/// Tick interval of the internal session ticker.
const INTERNAL_TICK_INTERVAL: Duration = Duration::from_millis(1000 / 20);
/// Tick interval for session packet processing.
const TICK_INTERVAL: Duration = Duration::from_millis(1000 / 20);
/// Inactivity timeout.
///
/// Any sessions that do not respond within this specified timeout will be disconnect from the server.
/// Timeouts can happen if a client's game crashed for example.
/// They will stop responding to the server, but will not explicitly send a disconnect request.
/// Hence, they have to be disconnected manually after the timeout passes.
const SESSION_TIMEOUT: Duration = Duration::from_secs(5);

impl Session {
    pub fn start_ticker_job(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(INTERNAL_TICK_INTERVAL);

            while !self.active.is_cancelled() {
                match self.tick().await {
                    Ok(_) => (),
                    Err(e) => tracing::error!("{e}"),
                }
                interval.tick().await;
            }

            // Flush last acknowledgements before closing
            match self.flush_acknowledgements().await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!(
                        "Failed to flush last acknowledgements before session close"
                    );
                }
            }

            // Flush last packets before closing
            match self.flush().await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!(
                        "Failed to flush last packets before session close"
                    );
                }
            }
        });
    }

    pub fn start_packet_job(
        self: Arc<Self>,
        mut receiver: mpsc::Receiver<Bytes>,
    ) {
        tokio::spawn(async move {
            let mut broadcast_recv = self.broadcast.subscribe();

            while !self.active.is_cancelled() {
                tokio::select! {
                    pk = receiver.recv() => {
                        if let Some(pk) = pk {
                            match self.process_raw_packet(pk).await {
                                Ok(_) => (),
                                Err(e) => tracing::error!("{e}"),
                            }
                        }
                    },
                    pk = broadcast_recv.recv() => {
                        if let Ok(pk) = pk {
                            match self.process_broadcast(pk) {
                                Ok(_) => (),
                                Err(e) => tracing::error!("{e}"),
                            }
                        }
                    }
                };
            }
        });
    }

    /// Signals to the session that it needs to close.
    pub fn on_disconnect(&self) {
        if !self.is_active() {
            return
        }

        self.initialized.store(false, Ordering::SeqCst);

        todo!();
        // if let Ok(display_name) = self.get_display_name() {
        //     if let Ok(uuid) = self.get_uuid() {
        //         tracing::info!("{display_name} has disconnected");
        //         let _ = self.broadcast_others(TextMessage {
        //             message: format!("§e{display_name} has left the server."),
        //             message_type: MessageType::System,
        //             needs_translation: false,
        //             parameters: vec![],
        //             platform_chat_id: "".to_owned(),
        //             source_name: "".to_owned(),
        //             xuid: "".to_owned(),
        //         });

        //         let _ = self
        //             .broadcast_others(PlayerListRemove { entries: &[*uuid] });
        //     }
        // }
        self.active.cancel();
    }

    /// Performs tasks not related to packet processing
    pub async fn tick(&self) -> VResult<()> {
        let current_tick = self.current_tick.fetch_add(1, Ordering::SeqCst);

        // Session has timed out
        if Instant::now().duration_since(*self.raknet.last_update.read())
            > SESSION_TIMEOUT
        {
            self.on_disconnect();
        }

        self.flush().await?;
        Ok(())
    }
}
