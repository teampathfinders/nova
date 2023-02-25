use std::{sync::Arc, num::NonZeroU64};

use bevy_ecs::{prelude::*, query};
use common::Vector3f;
use uuid::Uuid;

use crate::network::{
    packets::{GameMode, MovePlayer, MovementMode, TeleportCause, login::{PermissionLevel, DeviceOS}},
    session::Session, Skin, raknet::{DEFAULT_SEND_CONFIG, BroadcastPacket},
};

#[derive(Component, Debug)]
pub struct Transform {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

#[derive(StageLabel)]
pub struct PlayerMoveLabel;

#[derive(Component, Debug)]
pub struct Player {
    pub username: String,
    pub xuid: u64,
    pub uuid: Uuid,
    /// Game mode.
    pub game_mode: GameMode,
    /// General permission level.
    pub permission_level: PermissionLevel,
    /// The client's skin.
    pub skin: Skin,
    pub session: Arc<Session>,
    pub device_os: DeviceOS
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub entity: ActiveEntity,
    pub player: Player,
    pub transform: Transform,
}

#[derive(Component)]
pub struct ActiveEntity {
    pub runtime_id: u64,
}

pub fn player_movement(
    mut writer: EventWriter<BroadcastPacket>, 
    query: Query<(&Player, &ActiveEntity, &Transform), Changed<Transform>>,
) {
    writer.send_batch(
        query
            .iter()
            .map(|(player, entity, transform)| {
                let pk = MovePlayer {
                    runtime_id: entity.runtime_id,
                    position: transform.position.clone(),
                    rotation: transform.rotation.clone(),
                    mode: MovementMode::Normal,
                    on_ground: true,
                    ridden_runtime_id: 0,
                    teleport_cause: TeleportCause::Unknown,
                    teleport_source_type: 0,
                    tick: 0,
                };

                BroadcastPacket::new(pk, Some(NonZeroU64::new(player.xuid).unwrap())).unwrap()
            })
    );
}

pub fn broadcast_event_handler(mut reader: EventReader<BroadcastPacket>, query: Query<&Player>) {
    for event in reader.iter() {
        for player in &query {
            if let Some(source) = event.sender {
                if source.get() != player.xuid {
                    player.session.send_serialized(event.content.clone(), DEFAULT_SEND_CONFIG);
                }
            }
        }
    }
}