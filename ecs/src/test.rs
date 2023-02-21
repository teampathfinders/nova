use common::Vector3f;

use crate::{Component, Entity, Query, World, Transform, System};

#[derive(Debug)]
pub struct Gravity {
    pub strength: f32,
}

impl Component for Gravity {}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, position) in &query {
        println!("Entity {entity:?} is at position {position:?}");
    }
}

#[test]
fn example() {
    let mut world = World::new();
    let entity = world.spawn((
        Transform {
            position: Vector3f::zero(),
            rotation: Vector3f::zero(),
        },
        Gravity { strength: 9.81 },
    ));

    world.system(print_position);
    world.despawn(entity);
}
