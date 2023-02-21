use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(usize);

pub struct Entities {
    /// List of entities.
    entities: Vec<Entity>,
    /// Maps entity IDs to indices in the entities vector.
    handle_map: HashMap<Entity, usize>,
    /// IDs that were freed by destroyed entities,
    /// these can be reused to keep the entities array packed.
    freed: Vec<usize>
}   

impl Entities {
    pub fn alloc(&mut self) -> Entity {
        todo!()
    }
}

impl Default for Entities {
    fn default() -> Entities {
        Entities {
            entities: Vec::new(),
            handle_map: HashMap::new(),
            freed: Vec::new()
        }
    }
}
