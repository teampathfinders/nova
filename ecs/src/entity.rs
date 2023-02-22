use std::num::NonZeroUsize;

/// An entity is just a unique ID.
/// The ID is nonzero so that the Rust compiler can use optimisations for the Option type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(NonZeroUsize);

/// List of currently alive entities.
pub(crate) struct Entities {
    /// List of entities.
    entities: Vec<Option<Entity>>,
}   

impl Entities {
    /// Registers a new entity.
    pub fn register(&mut self) -> Entity {
        // Check for gaps in the entities list.
        let free_id = self.entities
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.is_none() { Some(i) } else { None });

        if let Some(id) = free_id {
            let entity = Entity(unsafe {
                NonZeroUsize::new_unchecked(id + 1)
            });

            self.entities[id] = Some(entity);
            entity
        } 
        // No free IDs, push to back of list.
        else {
            // SAFETY: Vector length cannot be negative, therefore the ID will also never be 0.
            let id = unsafe {
                NonZeroUsize::new_unchecked(self.entities.len() + 1)
            };
            let entity = Entity(id);

            self.entities.push(Some(entity));
            entity
        }
    }

    /// Unregisters an entity previously created with [`register`](Entities::register).
    pub fn deregister(&mut self, entity: Entity) {
        self.entities[entity.0.get() - 1] = None;
    }
}

impl Default for Entities {
    fn default() -> Entities {
        Entities {
            entities: Vec::new()
        }
    }
}
