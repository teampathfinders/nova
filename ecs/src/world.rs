use crate::{Entities, Entity, Components, Collection};

pub struct World {
    entities: Entities,
    components: Components
}

impl World {
    pub fn new() -> World {
        World::default()
    }

    pub fn summon<C: Collection>(&mut self, components: C) -> Entity {
        let entity = self.entities.register();        
        components.insert(entity, &mut self.components);

        entity
    }       
}

impl Default for World {
    fn default() -> World {
        World {
            entities: Entities::default(),
            components: Components::default()
        }
    }
}