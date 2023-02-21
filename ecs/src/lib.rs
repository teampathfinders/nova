//! Entity component system library that tries to imitate the API of [`bevy_ecs`](https://docs.rs/bevy_ecs)
//! while containing custom features for the server.

use common::{glob_export, Vector3f};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

impl Component for Transform {}

glob_export!(world);
glob_export!(entity);
glob_export!(component);
glob_export!(query);
glob_export!(system);
