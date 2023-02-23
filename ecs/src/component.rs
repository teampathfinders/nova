use std::{any::{TypeId, Any}, collections::HashMap};

use crate::{Entity, QueryComponents, QueryFilters, Query, Entities};

/// Represents a component that can be queried by a system.
pub trait Component {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct ComponentId(TypeId);

/// A component index
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct ComponentIndex(usize);

/// Stores all components of a single type.
#[derive(Debug)]
pub(crate) struct ComponentStorage<C: Component> {
    /// Maps entity IDs to indices in storage.
    indices: HashMap<Entity, usize>,
    /// Component storage.
    storage: Vec<Option<C>>    
}

impl<C: Component> ComponentStorage<C> {
    pub fn new() -> ComponentStorage<C> {
        ComponentStorage {
            indices: HashMap::new(),
            storage: Vec::new()
        }
    }

    /// Stores a new component in the storage, linked to the given entity.
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
pub(crate) trait Storage {
    /// Returns the unique ID of the component.
    fn type_id(&self) -> TypeId;
    /// Casts self to [`Any`];
    fn as_any(&self) -> &dyn Any;
    /// Casts self to a mutable [`Any`].
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn has_entity(&self, entity: Entity) -> bool;
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

    #[inline]
    fn has_entity(&self, entity: Entity) -> bool {
        self.indices.contains_key(&entity)
    }

    fn deregister(&mut self, entity: Entity) {
        if let Some(index) = self.indices.remove(&entity) {
            self.storage[index] = None;
        }
    }
}

pub trait StorageQuery<C: Component> {
    fn query(&self, entity: Entity) -> Option<&C>;
}

impl<C: Component + 'static> StorageQuery<C> for ComponentStorage<C> {
    fn query(&self, entity: Entity) -> Option<&C> {
        self.storage.get(*self.indices.get(&entity)?)?.as_ref()
    }
}

/// Contains a list of components sorted by component ID.
pub struct Components {
    pub(crate) storage: HashMap<TypeId, Box<dyn Storage>>
}

impl Components {
    #[inline]
    pub(crate) fn query<Q: QueryComponents, F: QueryFilters>(&self, entities: &Entities) -> Query<Q, F> {
        Q::gather::<F>(entities, self)
    }

    pub(crate) fn query_mut<Q: QueryComponents, F: QueryFilters>(&self) -> Query<Q, F> {
        todo!()
    }

    /// Adds a component to the registry for the specified entity.
    pub(crate) fn register<C: Component + 'static>(&mut self, entity: Entity, component: C) {
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

    /// Removes all of an entity's components from the registry.
    pub(crate) fn deregister(&mut self, entity: Entity) {
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

/// Represents a collection of components.
pub trait Collection {
    /// Inserts the components into the component registry.
    fn insert(self, entity: Entity, registry: &mut Components);
}

impl Collection for () {
    fn insert(self, _entity: Entity, _registry: &mut Components) {
        // do nothing.
    }
}

impl<C0> Collection for C0 
    where C0: Component + 'static
{
    fn insert(self, entity: Entity, registry: &mut Components) {
        registry.register(entity, self);
    }
}

impl<C0, C1> Collection for (C0, C1)
    where
        C0: Component + 'static,
        C1: Component + 'static
{
    fn insert(self, entity: Entity, registry: &mut Components) {
        registry.register(entity, self.0);
        registry.register(entity, self.1);
    }
}

impl<C0, C1, C2> Collection for (C0, C1, C2) 
    where 
        C0: Component + 'static,
        C1: Component + 'static,
        C2: Component + 'static
{
    fn insert(self, entity: Entity, registry: &mut Components) {
        registry.register(entity, self.0);
        registry.register(entity, self.1);
        registry.register(entity, self.2);
    }
}

impl<C0, C1, C2, C3> Collection for (C0, C1, C2, C3) 
    where
        C0: Component + 'static,
        C1: Component + 'static,
        C2: Component + 'static,
        C3: Component + 'static
{
    fn insert(self, entity: Entity, registry: &mut Components) {
        registry.register(entity, self.0);
        registry.register(entity, self.1);
        registry.register(entity, self.2);
        registry.register(entity, self.3);
    }
}

impl<C0, C1, C2, C3, C4> Collection for (C0, C1, C2, C3, C4) 
    where
        C0: Component + 'static,
        C1: Component + 'static,
        C2: Component + 'static,
        C3: Component + 'static,
        C4: Component + 'static
{
    fn insert(self, entity: Entity, registry: &mut Components) {
        registry.register(entity, self.0);
        registry.register(entity, self.1);
        registry.register(entity, self.2);
        registry.register(entity, self.3);
        registry.register(entity, self.4);
    }
}