use std::marker::PhantomData;

use crate::{Component, Entity};

#[derive(Debug)]
pub struct QueryMeta {
    pub test: String
}

pub trait ComponentQuery {
    /// Whether the implementor requires a mutable reference.
    /// This determines whether the system is shared or exclusive.
    const MUTABLE: bool;
}

impl ComponentQuery for Entity {
    const MUTABLE: bool = false;
}

impl<C: Component> ComponentQuery for &C {
    const MUTABLE: bool = false;
}

impl<C: Component> ComponentQuery for &mut C {
    const MUTABLE: bool = true;
}

impl<Q0, Q1> ComponentQuery for (Q0, Q1)
    where Q0: ComponentQuery, Q1: ComponentQuery
{
    const MUTABLE: bool = Q0::MUTABLE || Q1::MUTABLE;
}

impl<Q0, Q1, Q2> ComponentQuery for (Q0, Q1, Q2)
    where Q0: ComponentQuery, Q1: ComponentQuery, Q2: ComponentQuery
{
    const MUTABLE: bool = Q0::MUTABLE || Q1::MUTABLE || Q2::MUTABLE;
}

impl<Q0, Q1, Q2, Q3> ComponentQuery for (Q0, Q1, Q2, Q3)
    where 
        Q0: ComponentQuery, Q1: ComponentQuery, 
        Q2: ComponentQuery, Q3: ComponentQuery
{
    const MUTABLE: bool = Q0::MUTABLE || Q1::MUTABLE || Q2::MUTABLE || Q3::MUTABLE;
}

impl<Q0, Q1, Q2, Q3, Q4> ComponentQuery for (Q0, Q1, Q2, Q3, Q4)
    where 
        Q0: ComponentQuery, Q1: ComponentQuery, Q2: ComponentQuery, 
        Q3: ComponentQuery, Q4: ComponentQuery
{
    const MUTABLE: bool = Q0::MUTABLE || Q1::MUTABLE || Q2::MUTABLE || Q3::MUTABLE || Q4::MUTABLE;
}

pub struct Changed<C: Component> {
    _marker: PhantomData<C>
}

pub struct With<C: Component> {
    _marker: PhantomData<C>
}

pub struct Without<C: Component> {
    _marker: PhantomData<C>
}

pub trait QueryFilter {}

impl QueryFilter for () {}
impl<C: Component> QueryFilter for Changed<C> {}
impl<C: Component> QueryFilter for With<C> {}
impl<C: Component> QueryFilter for Without<C> {}

pub struct Query<Q: ComponentQuery, F: QueryFilter = ()> {
    query: Vec<Option<Q>>,
    index: usize,
    _marker: PhantomData<F>
}

impl<Q: ComponentQuery, F: QueryFilter> Query<Q, F> {
    pub const fn exclusive() -> bool {
        Q::MUTABLE
    }

    pub const fn shared() -> bool {
        !Q::MUTABLE
    }

    pub(crate) fn empty() -> Query<Q, F> {
        Query {
            query: Vec::new(),
            index: 0,
            _marker: PhantomData
        }
    }

    pub(crate) fn meta() -> QueryMeta {
        QueryMeta {
            test: format!("Q: {}, F: {}", std::any::type_name::<Q>(), std::any::type_name::<F>())
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
}

// impl<Q: ComponentQuery, F: QueryFilter> Iterator for Query<'_, Q, F> {
//     type Item = Q;

//     fn next(&mut self) -> Option<Q> {
//         let iter = self.query.into_iter();
//         let next = iter.next();

//         todo!()
//     }
// }