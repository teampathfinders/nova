use crate::{Entities, Entity, Components, Collection, Systems, System, QueryComponents, Query, QueryFilters, EntityRef, SystemVariant};

pub struct World {
    entities: Entities,
    components: Components,
    systems: Systems
}

impl World {
    /// Creates a new, empty world.
    pub fn new() -> World {
        World::default()
    }

    /// Spawns an entity without components.
    /// This is the same calling [`spawn`](Self::spawn) with a unit type.
    pub fn spawn_empty(&mut self) -> EntityRef {
        let entity = self.entities.register();

        EntityRef {
            entity,
            world: self
        }
    }

    /// Summons a new entity with the given components.
    pub fn spawn<C: Collection>(&mut self, collection: C) -> EntityRef {
        let entity = self.entities.register();  
        collection.insert(entity, &mut self.components);

        EntityRef {
            entity,
            world: self
        }
    }

    /// Despawns an entity previously created with [`spawn`](Self::spawn) or [`spawn_empty`](Self::spawn_empty).
    pub fn despawn<E: Into<Entity>>(&mut self, entity: E) {
        let entity = entity.into();

        self.entities.deregister(entity);
        self.components.deregister(entity);
    }

    pub fn system<Q: QueryComponents + 'static, F: QueryFilters + 'static, S: Fn(Query<Q, F>) + 'static>(&mut self, system: S) {
        self.systems.register(system);
    }

    pub fn execute(&mut self) {
        self.systems.iter_mut().for_each(|system| {
            match system.variant() {
                SystemVariant::Shared => {
                    system.call(&self.entities, &self.components);
                },
                SystemVariant::Exclusive => {
                    system.call_mut(&mut self.components);
                }
            }
        })
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