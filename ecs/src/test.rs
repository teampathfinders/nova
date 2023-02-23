use common::Vector3f;

use crate::{Changed, Component, Entity, Query, With, Without, World};

#[derive(Debug)]
pub struct Gravity {
    pub strength: f32,
}

impl Component for Gravity {}

fn print_position(query: Query<&Transform>) {
    println!("Position system");

    for transform in query {
        println!("Entity is at position {transform:?}");
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

    world.spawn(Transform {
        position: Vector3f::zero(),
        rotation: Vector3f::zero(),
    });
    world.system(print_position);

    world.execute();
}
