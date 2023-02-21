use std::{any::{TypeId, Any}, collections::HashMap};

use crate::Entity;

pub trait Component {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(usize);

#[derive(Debug)]
pub struct ComponentStorage<C: Component> {
    indices: HashMap<Entity, usize>,
    storage: Vec<Option<C>>    
}

impl<C: Component> ComponentStorage<C> {
    pub fn new() -> ComponentStorage<C> {
        ComponentStorage {
            indices: HashMap::new(),
            storage: Vec::new()
        }
    }

    pub fn register(&mut self, entity: Entity, component: C) {
        // Find gaps
        let gap = self.storage
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.is_none() { Some(i) } else { None });

        let id = if let Some(id) = gap {
            self.storage[id] = Some(component);
            id
        } else {
            self.storage.push(Some(component));
            self.storage.len() - 1
        };

        self.indices.insert(entity, id);
    } 
}

/// Abstraction over a specific component storage.
/// This allows [`Components`] to store them.
pub trait Storage {
    fn type_id(&self) -> TypeId;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Deregister is put in the trait so downcasting is not needed.
    /// This is not possible with [`register`](ComponentStorage::register) because
    /// it contains a generic parameter.
    fn deregister(&mut self, entity: Entity);
}

impl<C: Component + 'static> Storage for ComponentStorage<C> {
    #[inline]
    fn type_id(&self) -> TypeId {
        TypeId::of::<C>()
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn deregister(&mut self, entity: Entity) {
        if let Some(index) = self.indices.remove(&entity) {
            self.storage[index] = None;
        }
    }
}

/// Contains a list of components sorted by component ID.
pub struct Components {
    storage: HashMap<TypeId, Box<dyn Storage>>
}

impl Components {
    pub fn register<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let type_id = TypeId::of::<C>();
        let entry = self.storage
            .entry(type_id)
            .or_insert_with(|| Box::new(ComponentStorage::<C>::new()));

        let storage = entry.as_any_mut().downcast_mut::<ComponentStorage<C>>();
        if storage.is_none() {
            unreachable!();
        }

        storage.unwrap().register(entity, component);
    }

    pub fn deregister(&mut self, entity: Entity) {
        for (_, v) in self.storage.iter_mut() {
            v.deregister(entity);
        }
    }
}

impl Default for Components {
    fn default() -> Components {
        Components {
            storage: HashMap::new()
        }
    }
}

pub trait Collection {
    fn insert(self, entity: Entity, registry: &mut Components);
}

impl<C: Component + 'static> Collection for C {
    fn insert(self, entity: Entity, registry: &mut Components) {
        registry.register(entity, self);
    }
}

impl<C0: Component + 'static, C1: Component + 'static> Collection for (C0, C1) {
    fn insert(self, entity: Entity, registry: &mut Components) {
        registry.register(entity, self.0);
        registry.register(entity, self.1);
    }
}