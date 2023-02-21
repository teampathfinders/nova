//! Entity component system library that tries to imitate the API of [`bevy_ecs`](https://docs.rs/bevy_ecs)
//! while containing custom features for the server.

use common::glob_export;

#[cfg(test)]
mod test;

glob_export!(entity);
glob_export!(query);
glob_export!(system);
glob_export!(world);
