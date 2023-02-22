use std::marker::PhantomData;

use crate::{ComponentQuery, QueryFilter, Query, QueryMeta};

pub enum SystemVariant {
    Exclusive,
    Shared
}

pub trait System {
    // /// Returns a description of the components and entities that the query has requested.
    fn query(&self) -> QueryMeta;
    fn variant(&self) -> SystemVariant;
}

/// Trait to convert a [`SystemCallable`] into a boxed [`System`].
pub trait IntoSystem {
    type System: System;

    /// Converts the callable into a system.
    fn into_system(self) -> Box<dyn System>;
}

/// SharedSystem wraps a system so that the System trait can be implemented for it.
/// 
/// This type should be used for systems that only query immutable reference to components.
/// The scheduler is then able to run multiple SharedSystems at the same time on different threads.
/// 
/// Implementing the System trait directly for function pointers doesn't work.
/// This is due to every closure and function pointer having a different type.
/// Structs have the same type and therefore can be used for the System trait.
pub struct SharedSystem<Q: ComponentQuery, F: QueryFilter, S: Fn(Query<Q, F>)> {
    callable: S,
    _q: PhantomData<Q>,
    _f: PhantomData<F>
}

impl<Q: ComponentQuery, F: QueryFilter, S: Fn(Query<Q, F>)> System for SharedSystem<Q, F, S> {
    fn variant(&self) -> SystemVariant {
        SystemVariant::Shared
    }

    fn query(&self) -> QueryMeta {
        Query::<Q, F>::meta()
    }

    // fn call(&self) {
    //     let meta = Query::<Q, F>::meta();
    //     dbg!(meta);

    //     (self.callable)(Query::empty())
    // }
}

impl<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: Fn(Query<Q, F>) + 'static> IntoSystem for SharedSystem<Q, F, S> {
    type System = SharedSystem<Q, F, S>;

    fn into_system(self) -> Box<dyn System> {
        Box::new(self)
    }
}

/// ExclusiveSystem wraps a system so that the System trait can be implemented for it.
/// 
/// Unlike [`SharedSystem`], this system will take exclusive access to the queried components
/// in order to be able to mutate them.
/// 
/// Implementing the System trait directly for function pointers doesn't work.
/// This is due to every closure and function pointer having a different type.
/// Structs have the same type and therefore can be used for the System trait.
pub struct ExclusiveSystem<Q: ComponentQuery, F: QueryFilter, S: FnMut(Query<Q, F>)> {
    callable: S,
    // The Q and F generics are used in the function trait, but the compiler still complains about them...
    _q: PhantomData<Q>,
    _f: PhantomData<F>
}

impl<Q: ComponentQuery, F: QueryFilter, S: FnMut(Query<Q, F>)> System for ExclusiveSystem<Q, F, S> {
    fn variant(&self) -> SystemVariant {
        SystemVariant::Exclusive
    }

    fn query(&self) -> QueryMeta {
        Query::<Q, F>::meta()
    }
}

impl<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: FnMut(Query<Q, F>) + 'static> IntoSystem for ExclusiveSystem<Q, F, S> {
    type System = ExclusiveSystem<Q, F, S>;

    fn into_system(self) -> Box<dyn System> {
        Box::new(self)
    }
}

/// Keeps track of the currently active systems.
pub struct Systems {
    systems: Vec<Box<dyn System>>
}

impl Systems {
    pub fn register<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: Fn(Query<Q, F>) + 'static>(&mut self, callable: S) {
        let system = if Query::<Q, F>::exclusive() {
            dbg!("System is exclusive");

            ExclusiveSystem {
                callable,
                _q: PhantomData,
                _f: PhantomData
            }.into_system()
        } else {
            dbg!("System is shared");

            SharedSystem {
                callable,
                _q: PhantomData,
                _f: PhantomData
            }.into_system()
        };

        self.systems.push(system);
    }

    pub fn call_all(&self) {
        // self.systems.iter().for_each(|s| {
        //     // s.call();
        //     // let meta = s.query();
        //     // dbg!(meta);

        //     // let callable = 
        // });
    }
}

impl Default for Systems {
    fn default() -> Systems {
        Systems {
            systems: Vec::new()
        }
    }
}