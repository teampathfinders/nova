use std::marker::PhantomData;

use crate::{Component, Entity};

#[derive(Debug)]
pub struct QueryMeta {
    pub test: String
}

pub trait ComponentQuery {}

impl ComponentQuery for () {}
impl ComponentQuery for Entity {}

impl<C: Component> ComponentQuery for &C {}
impl<C: Component> ComponentQuery for &mut C {}

impl<F0: ComponentQuery, F1: ComponentQuery> ComponentQuery for (F0, F1) {}

pub struct Changed<C: Component> {
    _marker: PhantomData<C>
}

pub trait QueryFilter {}

impl QueryFilter for () {}
impl<C: Component> QueryFilter for Changed<C> {}

pub struct Query<'a, Q: ComponentQuery, F: QueryFilter = ()> {
    query: &'a [Q],
    index: usize,
    _marker: PhantomData<F>
}

impl<Q: ComponentQuery, F: QueryFilter> Query<'_, Q, F> {
    pub fn meta() -> QueryMeta {
        QueryMeta {
            test: format!("Q: {}, F: {}", std::any::type_name::<Q>(), std::any::type_name::<F>())
        }
    }
}

impl<Q: ComponentQuery + Copy, F: QueryFilter> ExactSizeIterator for &Query<'_, Q, F> {
    fn len(&self) -> usize {
        self.query.len() - self.index - 1
    }
}

impl<Q: ComponentQuery + Copy, F: QueryFilter> Iterator for &Query<'_, Q, F> {
    type Item = Q;

    fn next(&mut self) -> Option<Q> {
        self.query.get(self.index).map(|q| *q)
    }
}