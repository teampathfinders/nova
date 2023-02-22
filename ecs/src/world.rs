use crate::{Entities, Entity, Components, Collection, Systems, System, ComponentQuery, Query, QueryFilter};

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
    pub fn spawn(&mut self, collection: impl Collection) -> Entity {
        let entity = self.entities.register();        
        collection.insert(entity, &mut self.components);

        entity
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.entities.deregister(entity);
        self.components.deregister(entity);
    }

    pub fn system<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: Fn(Query<Q, F>) + 'static>(&mut self, system: S) {
        self.systems.register(system);
    }

    pub fn execute(&self) {
        self.systems.call_all();
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