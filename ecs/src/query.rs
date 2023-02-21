use std::marker::PhantomData;

use crate::{Component, Entity};

pub trait ReadableQuery {

}

pub trait ComponentQuery {}

impl ComponentQuery for () {}
impl ComponentQuery for Entity {}

impl<C: Component> ComponentQuery for &C {}
impl<C: Component> ComponentQuery for &mut C {}

impl<F0: ComponentQuery, F1: ComponentQuery> ComponentQuery for (F0, F1) {}

pub trait QueryFilter {}

impl QueryFilter for () {}

pub struct Query<'a, Q: ComponentQuery, F: QueryFilter = ()> {
    query: &'a [Q],
    index: usize,
    _marker: PhantomData<F>
}

impl<Q: ComponentQuery + Copy> ExactSizeIterator for &Query<'_, Q> {
    fn len(&self) -> usize {
        self.query.len() - self.index - 1
    }
}

impl<Q: ComponentQuery + Copy> Iterator for &Query<'_, Q> {
    type Item = Q;

    fn next(&mut self) -> Option<Q> {
        self.query.get(self.index).map(|q| *q)
    }
}

/// Queries all entities with the given component T.
/// An immutable reference is returned.
impl<'a, T: Component> ReadableQuery for Query<'a, (Entity, &T)> {
    
}

/// Queries all entities with the given component T.
/// A mutable refernence is returned.
impl<'a, T: Component> ReadableQuery for Query<'a, (Entity, &mut T)> {

}