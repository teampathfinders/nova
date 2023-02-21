use std::marker::PhantomData;

use crate::{ComponentQuery, QueryFilter, Query};

pub trait System {}

impl<Q: ComponentQuery, F: QueryFilter, S: FnMut(Query<Q, F>)> System for SystemCallable<Q, F, S> {}

pub struct SystemCallable<Q: ComponentQuery, F: QueryFilter, S: FnMut(Query<Q, F>)> {
    f: S,
    _q: PhantomData<Q>,
    _f: PhantomData<F>
}

pub trait IntoSystem<Q: ComponentQuery, F: QueryFilter> {
    type System: System;

    fn into_system(this: Self) -> Box<dyn System>;
}

impl<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: FnMut(Query<Q, F>) + 'static> IntoSystem<Q, F> for SystemCallable<Q, F, S> {
    type System = SystemCallable<Q, F, S>;

    fn into_system(this: Self) -> Box<dyn System> {
        Box::new(this)
    }
}

pub struct Systems {
    systems: Vec<Box<dyn System>>
}

impl Systems {
    pub fn register<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: FnMut(Query<Q, F>) + 'static>(&mut self, system: S) {
        let simple = SystemCallable {
            f: system,
            _q: PhantomData,
            _f: PhantomData
        };

        let system = SystemCallable::into_system(simple);
        self.systems.push(system);
    }
}

impl Default for Systems {
    fn default() -> Systems {
        Systems {
            systems: Vec::new()
        }
    }
}