use std::sync::Arc;

use bevy_ecs::{prelude::*, query};
use common::Vector3f;

use crate::network::{
    packets::{GameMode, MovePlayer, MovementMode, TeleportCause},
    session::Session,
};

#[derive(Component)]
pub struct Transform {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

#[derive(StageLabel)]
pub struct PlayerMoveLabel;

#[derive(Component)]
pub struct Player {
    pub session: Arc<Session>,
    pub game_mode: GameMode,
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
    query: Query<(&Player, &ActiveEntity, &Transform), Changed<Transform>>,
) {
    for (player, entity, transform) in &query {
        player.session.broadcast(MovePlayer {
            runtime_id: entity.runtime_id,
            position: transform.position.clone(),
            rotation: transform.rotation.clone(),
            mode: MovementMode::Normal,
            on_ground: true,
            ridden_runtime_id: 0,
            teleport_cause: TeleportCause::Unknown,
            teleport_source_type: 0,
            tick: 0,
        });
    }
}
