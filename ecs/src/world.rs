use std::sync::atomic::{AtomicUsize, Ordering};

use parking_lot::RwLock;

use crate::{Entity, Component, Bundle};

pub struct World {
    latest_id: AtomicUsize,
    entities: RwLock<Vec<Entity>>
}

impl World {
    pub fn new() -> Self {
        Self {
            latest_id: AtomicUsize::new(0),
            entities: RwLock::new(Vec::new())
        }
    }

    pub fn summon<B: Bundle>(&self, components: B) -> Entity {
        let entity = Entity::from(
            self.latest_id.fetch_add(1, Ordering::Relaxed)
        );
        
        self.entities.write().push(entity);
        entity
    }
}
