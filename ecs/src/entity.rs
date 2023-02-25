use std::num::NonZeroUsize;

use crate::{World, Component};

#[derive(Copy, Clone)]
pub struct EntityRef<'w> {
    pub(crate) entity: Entity,
    pub(crate) world: &'w World
}

impl EntityRef<'_> {
    pub fn id(self) -> Entity {
        self.entity
    }
}

impl From<EntityRef<'_>> for Entity {
    fn from(value: EntityRef<'_>) -> Entity {
        value.entity
    }
}

/// An entity is just a unique ID.
/// The ID is nonzero so that the Rust compiler can use optimisations for the Option type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Entity(NonZeroUsize);

impl Component for Entity {}

/// List of currently alive entities.
#[derive(Debug)]
pub struct Entities {
    /// List of entities.
    pub(crate) storage: Vec<Option<Entity>>,
}   

impl Entities {
    /// Registers a new entity.
    pub fn register(&mut self) -> Entity {
        // Check for gaps in the entities list.
        let free_id = self.storage
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.is_none() { Some(i) } else { None });

        if let Some(id) = free_id {
            let entity = Entity(unsafe {
                NonZeroUsize::new_unchecked(id + 1)
            });

            self.storage[id] = Some(entity);
            entity
        } 
        // No free IDs, push to back of list.
        else {
            // SAFETY: Vector length cannot be negative, therefore the ID will also never be 0.
            let id = unsafe {
                NonZeroUsize::new_unchecked(self.storage.len() + 1)
            };
            let entity = Entity(id);

            self.storage.push(Some(entity));
            entity
        }
    }

    /// Unregisters an entity previously created with [`register`](Entities::register).
    pub fn deregister(&mut self, entity: Entity) {
        self.storage[entity.0.get() - 1] = None;
    }
    
    pub fn iter(&self) -> EntityIter {
        EntityIter::new(self)
    }
}

impl Default for Entities {
    fn default() -> Entities {
        Entities {
            storage: Vec::new()
        }
    }
}

pub struct EntityIter<'a> {
    index: usize,
    entities: &'a Entities
}

impl<'a> EntityIter<'a> {
    pub(crate) fn new(entities: &'a Entities) -> Self {
        Self {
            index: 0,
            entities
        }
    }
}

impl ExactSizeIterator for EntityIter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.entities.storage.len() - self.index
    }
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        self.index += 1;
        *self.entities.storage.get(self.index)?
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.entities.storage.len() - self.index))
    }
}