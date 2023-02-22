use std::{marker::PhantomData, any::TypeId};

use crate::{Component, Entity};

#[derive(Debug)]
pub enum QueryFilterVariant {
    None,
    With(TypeId),
    Without(TypeId),
    Changed(TypeId),
    Collection(&'static [QueryFilterVariant])
}

#[derive(Debug)]
pub(crate) struct QueryDescriptor {
    filters: &'static [QueryFilterVariant]
}

/// Coupled with [`Query`], this specifies the list of components to request for the system.
/// [`More info`](Query).
pub trait ComponentQuery {
    /// Whether the implementor requires a mutable reference.
    /// This determines whether the system is shared or exclusive.
    const EXCLUSIVE: bool;
}

impl ComponentQuery for Entity {
    const EXCLUSIVE: bool = false;
}

impl<C: Component> ComponentQuery for &C {
    const EXCLUSIVE: bool = false;
}

impl<C: Component> ComponentQuery for &mut C {
    const EXCLUSIVE: bool = true;
}

impl<Q0, Q1> ComponentQuery for (Q0, Q1)
    where Q0: ComponentQuery, Q1: ComponentQuery
{
    const EXCLUSIVE: bool = Q0::EXCLUSIVE || Q1::EXCLUSIVE;
}

impl<Q0, Q1, Q2> ComponentQuery for (Q0, Q1, Q2)
    where Q0: ComponentQuery, Q1: ComponentQuery, Q2: ComponentQuery
{
    const EXCLUSIVE: bool = Q0::EXCLUSIVE || Q1::EXCLUSIVE || Q2::EXCLUSIVE;
}

impl<Q0, Q1, Q2, Q3> ComponentQuery for (Q0, Q1, Q2, Q3)
    where 
        Q0: ComponentQuery, Q1: ComponentQuery, 
        Q2: ComponentQuery, Q3: ComponentQuery
{
    const EXCLUSIVE: bool = Q0::EXCLUSIVE || Q1::EXCLUSIVE || Q2::EXCLUSIVE || Q3::EXCLUSIVE;
}

impl<Q0, Q1, Q2, Q3, Q4> ComponentQuery for (Q0, Q1, Q2, Q3, Q4)
    where 
        Q0: ComponentQuery, Q1: ComponentQuery, Q2: ComponentQuery, 
        Q3: ComponentQuery, Q4: ComponentQuery
{
    const EXCLUSIVE: bool = Q0::EXCLUSIVE || Q1::EXCLUSIVE || Q2::EXCLUSIVE || Q3::EXCLUSIVE || Q4::EXCLUSIVE;
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

/// Applies a filter to a component query.
/// 
/// This can be used to filter certain components and entities from the request.
/// Some available filters are [`Changed`], [`With`] and [`Without`].
pub trait QueryFilter {
    const VARIANTS: &'static [QueryFilterVariant];
    const COUNT: usize;
}

impl QueryFilter for () {
    const VARIANTS: &'static [QueryFilterVariant] = &[QueryFilterVariant::None];
    const COUNT: usize = 0;
}

impl<C: Component + 'static> QueryFilter for Changed<C> {
    const VARIANTS: &'static [QueryFilterVariant] = &[QueryFilterVariant::Changed(TypeId::of::<C>())];
    const COUNT: usize = 1;
}

impl<C: Component + 'static> QueryFilter for With<C> {
    const VARIANTS: &'static [QueryFilterVariant] = &[QueryFilterVariant::With(TypeId::of::<C>())];
    const COUNT: usize = 1;
}

impl<C: Component + 'static> QueryFilter for Without<C> {
    const VARIANTS: &'static [QueryFilterVariant] = &[QueryFilterVariant::Without(TypeId::of::<C>())];
    const COUNT: usize = 1;
}

impl<F0, F1> QueryFilter for (F0, F1) 
    where F0: QueryFilter, F1: QueryFilter
{
    const VARIANTS: &'static [QueryFilterVariant] = &[QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F1::VARIANTS)];
    const COUNT: usize = F0::COUNT + F1::COUNT;
}

impl<F0, F1, F2> QueryFilter for (F0, F1, F2) 
    where F0: QueryFilter, F1: QueryFilter, F2: QueryFilter
{
    const VARIANTS: &'static [QueryFilterVariant] = &[QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS)];
    const COUNT: usize = F0::COUNT + F1::COUNT + F2::COUNT;
}

impl<F0, F1, F2, F3> QueryFilter for (F0, F1, F2, F3) 
    where F0: QueryFilter, F1: QueryFilter, F2: QueryFilter, F3: QueryFilter
{
    const VARIANTS: &'static [QueryFilterVariant] = &[QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS)];
    const COUNT: usize = F0::COUNT + F1::COUNT + F2::COUNT + F3::COUNT;
}

impl<F0, F1, F2, F3, F4> QueryFilter for (F0, F1, F2, F3, F4) 
    where F0: QueryFilter, F1: QueryFilter, F2: QueryFilter, F3: QueryFilter, F4: QueryFilter
{
    const VARIANTS: &'static [QueryFilterVariant] = &[QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS), QueryFilterVariant::Collection(F0::VARIANTS)];
    const COUNT: usize = F0::COUNT + F1::COUNT + F2::COUNT + F3::COUNT + F4::COUNT;
}

/// Queries are used by systems to request certain components.
/// 
/// # Generics
/// * `Q` - The [`ComponentQuery`] generic determines which components to request.
///         This can either be a singular component or a tuple of components.
/// * `F` - The [`QueryFilter`] generic determines the filters to apply to the request.
///         Using the filters, you can for example execute a system only if some entity's 
///         component has changed or request all entities that have component A, but not B.
///         By default, this filter is set to a unit, which doesn't apply anything.
/// 
/// The kind of references that the query requests in the [`ComponentQuery`] argument decides 
/// how the system will be scheduled.
/// *Immutable references* allow the scheduler to run this system in parallel with other systems
/// that also immutably request the same component(s).<br/>
/// *Mutable references* require exclusive access to the component and, hence, run sequentially.
pub struct Query<Q: ComponentQuery, F: QueryFilter = ()> {
    query: Vec<Option<Q>>,
    /// Current iterator index.
    index: usize,
    #[doc(hidden)]
    _marker: PhantomData<F>
}

impl<Q: ComponentQuery, F: QueryFilter> Query<Q, F> {
    /// Whether this query requests mutable (and therefore exclusive) access to
    /// one or more components.
    pub(crate) const fn exclusive() -> bool {
        Q::EXCLUSIVE
    }

    /// Whether this query only requests immutable references.
    pub(crate) const fn shared() -> bool {
        !Q::EXCLUSIVE
    }

    pub(crate) fn empty() -> Query<Q, F> {
        Query {
            query: Vec::new(),
            index: 0,
            _marker: PhantomData
        }
    }

    pub(crate) fn meta() -> QueryDescriptor {
        QueryDescriptor {
            filters: F::VARIANTS
        }
    }
}

impl<Q: ComponentQuery, F: QueryFilter> ExactSizeIterator for Query<Q, F> {
    #[inline]
    fn len(&self) -> usize {
        self.query.len() - self.index
    }
}

impl<Q: ComponentQuery, F: QueryFilter> Iterator for Query<Q, F> {
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