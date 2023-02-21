use common::Vector3f;

use crate::{Component, Entity, Query, World};

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

impl Component for Transform {}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, position) in query {
        println!("Entity {entity:?} is at position {position:?}");
    }
}

#[test]
fn example() {
    let world = World::new();
    let entity = world.summon(Transform {
        position: Vector3f::zero(),
        rotation: Vector3f::zero(),
    });
}
