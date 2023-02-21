use crate::{Entities, Entity, Components, Collection, ComponentQuery, QueryFilter, Query, Systems};

pub struct World {
    entities: Entities,
    components: Components,
    systems: Systems
}

impl World {
    pub fn new() -> World {
        World::default()
    }

    pub fn spawn_empty(&mut self) -> Entity {
        self.entities.register()
    }

    /// Summons a new entity with the given components.
    pub fn spawn<C: Collection>(&mut self, collection: C) -> Entity {
        let entity = self.entities.register();        
        collection.insert(entity, &mut self.components);

        entity
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.entities.deregister(entity);
        self.components.deregister(entity);
    }

    pub fn system<Q: ComponentQuery, F: QueryFilter, S: FnMut(Query<Q, F>)>(&mut self, system: S) {

    }   
}

impl Default for World {
    fn default() -> World {
        World {
            entities: Entities::default(),
            components: Components::default(),
            systems: Systems::default()
        }
    }
}