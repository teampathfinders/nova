//! Entity component system library that tries to imitate the API of [`bevy_ecs`](https://docs.rs/bevy_ecs)
//! while containing custom features for the server.

use common::glob_export;

#[cfg(test)]
mod test;

glob_export!(query);

#[derive(Debug, Copy, Clone)]
pub struct Entity {}

#[derive(Debug)]
pub struct World {}

impl World {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(&self) -> Entity {
        todo!()
    }
}

pub trait Component {}
