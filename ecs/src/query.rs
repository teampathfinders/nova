use std::{marker::PhantomData, any::TypeId};

use crate::{Component, Entity, Components, Entities};

#[derive(Debug)]
pub(crate) struct QueryDescriptor {
    pub(crate) components: &'static [QueryComponentDescriptor],
    pub(crate) filters: &'static [QueryFilterDescriptor]
}

#[derive(Debug)]
pub struct QueryComponentDescriptor {
    pub(crate) id: TypeId,
    pub(crate) mutable: bool
}

pub struct QueryCollector<Q: QueryComponents, F: QueryFilters> {
    collection: Vec<Option<Query<Q, F>>>
}

// /// Coupled with [`Query`], this specifies the list of components to request for the system.
// /// [`More info`](Query).
pub trait QueryComponents: Sized {
    /// Whether the implementor requires a mutable reference.
    /// This determines whether the system is shared or exclusive.
    const EXCLUSIVE: bool;
    const DESCRIPTORS: &'static [QueryComponentDescriptor];

    fn gather<F: QueryFilters>(entities: &Entities, components: &Components) -> QueryCollector<Self, F>;
}

trait SingularQueryComponent: QueryComponents {
    const SINGULAR_EXCLUSIVE: bool;
    const DESCRIPTOR: QueryComponentDescriptor;
}

impl SingularQueryComponent for Entity {
    const SINGULAR_EXCLUSIVE: bool = false;
    const DESCRIPTOR: QueryComponentDescriptor = QueryComponentDescriptor { id: TypeId::of::<Entity>(), mutable: false };
}

impl<C: Component + 'static> SingularQueryComponent for &C {
    const SINGULAR_EXCLUSIVE: bool = false;
    const DESCRIPTOR: QueryComponentDescriptor = QueryComponentDescriptor { id: TypeId::of::<C>(), mutable: false };
}

impl<C: Component + 'static> SingularQueryComponent for &mut C {
    const SINGULAR_EXCLUSIVE: bool = true;
    const DESCRIPTOR: QueryComponentDescriptor = QueryComponentDescriptor { id: TypeId::of::<C>(), mutable: true };
}

impl<Q0> QueryComponents for Q0
    where Q0: SingularQueryComponent
{
    const EXCLUSIVE: bool = Q0::SINGULAR_EXCLUSIVE;
    const DESCRIPTORS: &'static [QueryComponentDescriptor] = &[Q0::DESCRIPTOR];

    fn gather<F: QueryFilters>(entities: &Entities, components: &Components) -> QueryCollector<Self, F> {
        todo!()
    }
}

impl<Q0, Q1> QueryComponents for (Q0, Q1)
    where Q0: SingularQueryComponent, Q1: SingularQueryComponent
{
    const EXCLUSIVE: bool = Q0::EXCLUSIVE || Q1::EXCLUSIVE;
    const DESCRIPTORS: &'static [QueryComponentDescriptor] = &[Q0::DESCRIPTOR, Q1::DESCRIPTOR];

    fn gather<F: QueryFilters>(entities: &Entities, components: &Components) -> QueryCollector<Self, F> {
        todo!()
    }
}

impl<Q0, Q1, Q2> QueryComponents for (Q0, Q1, Q2)
    where Q0: SingularQueryComponent, Q1: SingularQueryComponent, Q2: SingularQueryComponent
{
    const EXCLUSIVE: bool = Q0::EXCLUSIVE || Q1::EXCLUSIVE || Q2::EXCLUSIVE;
    const DESCRIPTORS: &'static [QueryComponentDescriptor] = &[Q0::DESCRIPTOR, Q1::DESCRIPTOR, Q2::DESCRIPTOR];

    fn gather<F: QueryFilters>(entities: &Entities, components: &Components) -> QueryCollector<Self, F> {
        todo!()
    }
}

impl<Q0, Q1, Q2, Q3> QueryComponents for (Q0, Q1, Q2, Q3)
    where 
        Q0: SingularQueryComponent, Q1: SingularQueryComponent, 
        Q2: SingularQueryComponent, Q3: SingularQueryComponent
{
    const EXCLUSIVE: bool = Q0::EXCLUSIVE || Q1::EXCLUSIVE || Q2::EXCLUSIVE || Q3::EXCLUSIVE;
    const DESCRIPTORS: &'static [QueryComponentDescriptor] = &[Q0::DESCRIPTOR, Q1::DESCRIPTOR, Q2::DESCRIPTOR, Q3::DESCRIPTOR];

    fn gather<F: QueryFilters>(entities: &Entities, components: &Components) -> QueryCollector<Self, F> {
        todo!()
    }
}

impl<Q0, Q1, Q2, Q3, Q4> QueryComponents for (Q0, Q1, Q2, Q3, Q4)
    where 
        Q0: SingularQueryComponent, Q1: SingularQueryComponent, Q2: SingularQueryComponent, 
        Q3: SingularQueryComponent, Q4: SingularQueryComponent
{
    const EXCLUSIVE: bool = Q0::EXCLUSIVE || Q1::EXCLUSIVE || Q2::EXCLUSIVE || Q3::EXCLUSIVE || Q4::EXCLUSIVE;
    const DESCRIPTORS: &'static [QueryComponentDescriptor] = &[Q0::DESCRIPTOR, Q1::DESCRIPTOR, Q2::DESCRIPTOR, Q3::DESCRIPTOR, Q4::DESCRIPTOR];

    fn gather<F: QueryFilters>(entities: &Entities, components: &Components) -> QueryCollector<Self, F> {
        todo!()
    }
}

/// Filter that only queries components that have been modified.
pub struct Changed<C: Component> {
    _marker: PhantomData<C>
}

/// Filter that specifices that the queried entities must have this component.
pub struct With<C: Component> {
    _marker: PhantomData<C>
}

/// Filter that specifies that the queried entities must not have this component.
pub struct Without<C: Component> {
    _marker: PhantomData<C>
}

#[derive(Debug)]
pub enum QueryFilterVariant {
    Changed,
    With,
    Without
}

pub type QueryFilterDescriptor = (QueryFilterVariant, TypeId);

/// Applies a filter to a component query.
/// 
/// This can be used to filter certain components and entities from the request.
/// Some available filters are [`Changed`], [`With`] and [`Without`].
pub trait QueryFilters {
    const DESCRIPTORS: &'static [QueryFilterDescriptor];
}

impl QueryFilters for () {
    const DESCRIPTORS: &'static [QueryFilterDescriptor] = &[];
}

impl<F0> QueryFilters for F0 
    where F0: SingularQueryFilter
{
    const DESCRIPTORS: &'static [QueryFilterDescriptor] = &[F0::DESCRIPTOR];
}

impl<F0, F1> QueryFilters for (F0, F1) 
    where 
        F0: SingularQueryFilter, F1: SingularQueryFilter, 
{
    const DESCRIPTORS: &'static [QueryFilterDescriptor] = &[F0::DESCRIPTOR, F1::DESCRIPTOR];
}

impl<F0, F1, F2> QueryFilters for (F0, F1, F2) 
    where 
        F0: SingularQueryFilter, F1: SingularQueryFilter, 
        F2: SingularQueryFilter
{
    const DESCRIPTORS: &'static [QueryFilterDescriptor] = &[F0::DESCRIPTOR, F1::DESCRIPTOR, F2::DESCRIPTOR];
}

impl<F0, F1, F2, F3> QueryFilters for (F0, F1, F2, F3) 
    where 
        F0: SingularQueryFilter, F1: SingularQueryFilter, 
        F2: SingularQueryFilter, F3: SingularQueryFilter
{
    const DESCRIPTORS: &'static [QueryFilterDescriptor] = &[F0::DESCRIPTOR, F1::DESCRIPTOR, F2::DESCRIPTOR, F3::DESCRIPTOR];
}

impl<F0, F1, F2, F3, F4> QueryFilters for (F0, F1, F2, F3, F4) 
    where 
        F0: SingularQueryFilter, F1: SingularQueryFilter, 
        F2: SingularQueryFilter, F3: SingularQueryFilter, 
        F4: SingularQueryFilter
{
    const DESCRIPTORS: &'static [QueryFilterDescriptor] = &[F0::DESCRIPTOR, F1::DESCRIPTOR, F2::DESCRIPTOR, F3::DESCRIPTOR, F4::DESCRIPTOR];
}

trait SingularQueryFilter {
    const DESCRIPTOR: (QueryFilterVariant, TypeId);
}

impl<C: Component + 'static> SingularQueryFilter for Changed<C> {
    const DESCRIPTOR: (QueryFilterVariant, TypeId) = (QueryFilterVariant::Changed, TypeId::of::<C>());
}

impl<C: Component + 'static> SingularQueryFilter for With<C> {
    const DESCRIPTOR: (QueryFilterVariant, TypeId) = (QueryFilterVariant::With, TypeId::of::<C>());
}

impl<C: Component + 'static> SingularQueryFilter for Without<C> {
    const DESCRIPTOR: (QueryFilterVariant, TypeId) = (QueryFilterVariant::Without, TypeId::of::<C>());
}

/// Queries are used by systems to request certain components.
/// 
/// # Generics
/// * `Q` - The [`QueryComponents`] generic determines which components to request.
///         This can either be a singular component or a tuple of components.
/// * `F` - The [`QueryFilters`] generic determines the filters to apply to the request.
///         Using the filters, you can for example execute a system only if some entity's 
///         component has changed or request all entities that have component A, but not B.
///         By default, this filter is set to a unit, which doesn't apply anything.
/// 
/// The kind of references that the query requests in the [`ComponentQuery`] argument decides 
/// how the system will be scheduled.
/// *Immutable references* allow the scheduler to run this system in parallel with other systems
/// that also immutably request the same component(s).<br/>
/// *Mutable references* require exclusive access to the component and, hence, run sequentially.
pub struct Query<Q: QueryComponents, F: QueryFilters = ()> {
    query: Vec<Option<Q>>,
    /// Current iterator index.
    index: usize,
    #[doc(hidden)]
    _marker: PhantomData<F>
}

impl<Q: QueryComponents, F: QueryFilters> Query<Q, F> {
    // pub(crate) const DESCRIPTOR: QueryDescriptor = QueryDescriptor {
    //     components: Q::DESCRIPTORS,
    //     filters: F::DESCRIPTORS
    // };

    /// Whether this query requests mutable (and therefore exclusive) access to
    /// one or more components.
    pub(crate) const fn exclusive() -> bool {
        Q::EXCLUSIVE
    }

    /// Whether this query only requests immutable references.
    pub(crate) const fn shared() -> bool {
        !Q::EXCLUSIVE
    }

    pub(crate) const fn empty() -> Query<Q, F> {
        Query {
            query: Vec::new(),
            index: 0,
            _marker: PhantomData
        }
    }
}

impl<Q: QueryComponents, F: QueryFilters> From<Vec<Option<Q>>> for Query<Q, F> {
    fn from(query: Vec<Option<Q>>) -> Query<Q, F> {
        Query {
            query,
            index: 0,
            _marker: PhantomData            
        }
    }
}

impl<Q: QueryComponents, F: QueryFilters> ExactSizeIterator for Query<Q, F> {
    #[inline]
    fn len(&self) -> usize {
        self.query.len() - self.index
    }
}

impl<Q: QueryComponents, F: QueryFilters> Iterator for Query<Q, F> {
    type Item = Q;

    #[inline]
    fn next(&mut self) -> Option<Q> {
        self.index += 1;
        self.query.get_mut(self.index - 1)?.take()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.query.len() - self.index))
    }
}