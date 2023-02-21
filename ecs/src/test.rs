use crate::{Component, Entity, Query, World};

#[derive(Debug)]
struct Transform {
    position: [f32; 3],
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
    let entity = world.spawn();
}
