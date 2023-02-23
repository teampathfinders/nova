use common::Vector3f;

use crate::{Changed, Component, Entity, Query, With, Without, World};

#[derive(Debug)]
pub struct Gravity {
    pub strength: f32,
}

impl Component for Gravity {}

impl Component for [u8] {}

fn print_position(query: Query<(Entity, &Transform), Without<Gravity>>) {
    println!("Position system");

    for (entity, position) in query {
        println!("Entity {entity:?} is at position {position:?}");
    }
}

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

impl Component for Transform {}

#[test]
fn example() {
    let mut world = World::new();

    world.system(print_position);
    world.spawn(Transform {
        position: Vector3f::zero(),
        rotation: Vector3f::zero(),
    });

    world.execute();
}
