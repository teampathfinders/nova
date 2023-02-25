use std::{any::{TypeId, Any}, collections::HashMap, marker::PhantomData};

use crate::{Entity, QueryComponents, QueryFilters, Query, Entities, SingularQueryComponent};

/// Represents a component that can be queried by a system.
pub trait Component {}

impl<'a, T: Component> Component for &'a T {}
impl<'a, T: Component> Component for &'a mut T {}

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

pub trait QueryMarker {
    fn safe(&self);
}

pub struct QueryMarkerImpl<T: Component, F: QueryFilters> {
    pub content: Option<T>,
    pub _marker: PhantomData<F>
}

impl<Q: SingularQueryComponent, F: QueryFilters> QueryMarker for QueryMarkerImpl<Q, F> {
    fn safe(&self) {
        dbg!(std::any::type_name::<Q>());
        dbg!(std::any::type_name::<F>());
    }
}

pub trait QueryableStorage {
    // fn query<C: Component, F: QueryFilters>(&self, marker: &QueryMarkerImpl<C, F>) -> Option<C>;
    fn query<M: QueryMarker + ?Sized>(&self, marker: &M);
    /// Deregister is put in the trait so downcasting is not needed.
    /// This is not possible with [`register`](ComponentStorage::register) because
    /// it contains a generic parameter.
    fn deregister(&mut self, entity: Entity);
}

impl<'a, T: Component> QueryableStorage for ComponentStorage<T> {
    // fn query<Q: Component, F: QueryFilters>(&self, marker: &QueryMarkerImpl<Q, F>) -> Option<Q> {
    //     // debug_assert_eq!(TypeId::of::<C>(), TypeId::of::<T>());
    //     dbg!(std::any::type_name::<T>());

    //     todo!()
    // }
    fn query<M: QueryMarker + ?Sized>(&self, marker: &M) {
        marker.safe();
    }

    fn deregister(&mut self, entity: Entity) {
        todo!()
    }
}

impl<T: ?Sized> QueryableStorage for Box<T> where T: QueryableStorage {
    // fn query<Q: Component, F: QueryFilters>(&self, marker: &QueryMarkerImpl<Q, F>) -> Option<Q> {
    //     (**self).query(marker)
    // }

    fn query<M: QueryMarker + ?Sized>(&self, marker: &M) {
        (**self).query(marker);
    }

    fn deregister(&mut self, entity: Entity) {
        (**self).deregister(entity);
    }
}

pub trait SafeQueryableStorage {
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn erased_query(&self, marker: &dyn QueryMarker);
    /// Deregister is put in the trait so downcasting is not needed.
    /// This is not possible with [`register`](ComponentStorage::register) because
    /// it contains a generic parameter.
    fn deregister(&mut self, entity: Entity);
}

impl<T: 'static> SafeQueryableStorage for T where T: QueryableStorage {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn erased_query(&self, out: &dyn QueryMarker) {
        self.query(out);
    }

    fn deregister(&mut self, entity: Entity) {
        self.deregister(entity);
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
    pub(crate) storage: HashMap<TypeId, Box<dyn SafeQueryableStorage>>
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

        if let Some(entry) = entry.as_any_mut().downcast_mut::<ComponentStorage<C>>() {
            entry.storage.push(Some(component));
            entry.indices.insert(entity, entry.storage.len() - 1);
        }
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