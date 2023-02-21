use std::marker::PhantomData;

use crate::{ComponentQuery, QueryFilter, Query, QueryMeta};

pub trait System {
    fn query(&self) -> QueryMeta;
}

impl<Q: ComponentQuery, F: QueryFilter, S: FnMut(Query<Q, F>)> System for SystemCallable<Q, F, S> {
    fn query(&self) -> QueryMeta {
        Query::<Q, F>::meta()
    }
}

/// SystemCallable wraps a system so that the System trait can be implemented for it.
/// 
/// Implementing the System directly for function pointers doesn't work.
/// This is due to every closure and function pointer having a different type.
/// Structs have the system type and therefore can be used for the System trait.
pub struct SystemCallable<Q: ComponentQuery, F: QueryFilter, S: FnMut(Query<Q, F>)> {
    f: S,
    // The Q and F generics are used in the function trait, but the compiler still complains about them...
    _q: PhantomData<Q>,
    _f: PhantomData<F>
}

pub trait IntoCallable<Q: ComponentQuery, F: QueryFilter> {
    type System: System;

    fn into_callable(this: Self) -> Box<dyn System>;
}

impl<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: FnMut(Query<Q, F>) + 'static> IntoCallable<Q, F> for SystemCallable<Q, F, S> {
    type System = SystemCallable<Q, F, S>;

    fn into_callable(this: Self) -> Box<dyn System> {
        Box::new(this)
    }
}

pub struct Systems {
    systems: Vec<Box<dyn System>>
}

impl Systems {
    pub fn register<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: FnMut(Query<Q, F>) + 'static>(&mut self, system: S) {
        let converter = SystemCallable {
            f: system,
            _q: PhantomData,
            _f: PhantomData
        };

        let system = SystemCallable::into_callable(converter);
        self.systems.push(system);
    }

    pub fn execute_each(&self) {
        for system in &self.systems {
            println!("query: {:?}", system.query());
        }
    }
}

impl Default for Systems {
    fn default() -> Systems {
        Systems {
            systems: Vec::new()
        }
    }
}