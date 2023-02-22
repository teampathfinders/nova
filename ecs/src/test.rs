use common::Vector3f;

use crate::{Changed, Component, Entity, Query, With, Without, World};

#[derive(Debug)]
pub struct Gravity {
    pub strength: f32,
}

impl Component for Gravity {}

fn print_position(query: Query<(Entity, &Transform), Changed<Transform>>) {
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

#[derive(Debug)]
struct Player {
    name: String,
}

impl Component for Player {}

impl Player {
    fn kill(&self) {}
}

struct Health {
    pub amount: u32,
}

impl Component for Health {}

fn health_system(
    query: Query<(&Player, &mut Health), (With<Transform>, Without<Gravity>)>,
) {
    for (player, health) in query {
        health.amount -= 1;

        if health.amount == 0 {
            println!("{player:?} has died");
            health.amount = 20;
        }
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

    let entity = world.spawn(());

    world.system(print_position);
    world.system(health_system);

    world.execute();
    world.despawn(entity);
}
