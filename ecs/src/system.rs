use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use crate::{QueryComponents, QueryFilters, Query, Components, Entities};

/// Describes the variant of a system.
/// 
/// This affects how the system will be scheduled to run.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum SystemVariant {
    /// The system queries include a mutable reference.
    /// 
    /// There can only be one mutable reference to a component at a time,
    /// therefore this system variant will request exclusive access to the
    /// queried components.
    Exclusive,
    /// The system uses only immutable references.
    /// 
    /// This system variant can be scheduled together with other shared systems
    /// accessing the same component. Thus, multiple shared systems can be ran at the same time.
    Shared
}

pub(crate) trait System {
    /// The system variant.
    /// This can be ether of the values of [`SystemVariant`].
    /// 
    /// This is a function instead of a constant, because the trait would otherwise not be object safe.
    fn variant(&self) -> SystemVariant;
    /// Calls a shared system.
    /// The system itself then queries the world.
    fn call(&self, entities: &Entities, components: &Components) {
        unimplemented!()
    }
    /// Calls an exclusive system.
    /// The system itself then queries the world.
    fn call_mut(&mut self, components: &mut Components) {
        unimplemented!()
    }
}

/// Helper trait to convert a [`SharedSystem`] or [`ExclusiveSystem`] into a boxed [`System`].
pub(crate) trait IntoSystem {
    /// Converts the callable into a system that can be stored in a [`Systems`] structure.
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
/// 
/// The [`IntoSystem`] trait can be used to convert this into a boxed [`System`].
pub(crate) struct SharedSystem<Q: QueryComponents, F: QueryFilters, S: Fn(Query<Q, F>)> {
    /// The actual function pointer.
    callable: S,
    // The Q and F generics are used in the function trait, but the compiler still complains about them...
    #[doc(hidden)]
    _marker: PhantomData<(Q, F)>,
}

impl<Q: QueryComponents, F: QueryFilters, S: Fn(Query<Q, F>)> System for SharedSystem<Q, F, S> {
    fn variant(&self) -> SystemVariant {
        SystemVariant::Shared
    }

    fn call(&self, entities: &Entities, components: &Components) {
        let query = components.query::<Q, F>(entities);
        (self.callable)(query);
    }
}

impl<Q: QueryComponents + 'static, F: QueryFilters + 'static, S: Fn(Query<Q, F>) + 'static> IntoSystem for SharedSystem<Q, F, S> {
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
pub(crate) struct ExclusiveSystem<Q: QueryComponents, F: QueryFilters, S: FnMut(Query<Q, F>)> {
    callable: S,
    // The Q and F generics are used in the function trait, but the compiler still complains about them...
    #[doc(hidden)]
    _marker: PhantomData<(Q, F)>
}

impl<Q: QueryComponents, F: QueryFilters, S: FnMut(Query<Q, F>)> System for ExclusiveSystem<Q, F, S> {
    fn variant(&self) -> SystemVariant {
        SystemVariant::Exclusive
    }

    fn call_mut(&mut self, components: &mut Components) {
        let query = components.query_mut();
        (self.callable)(query);
    }
}

impl<Q: QueryComponents + 'static, F: QueryFilters + 'static, S: FnMut(Query<Q, F>) + 'static> IntoSystem for ExclusiveSystem<Q, F, S> {
    fn into_system(self) -> Box<dyn System> {
        Box::new(self)
    }
}

/// Keeps track of the currently active systems.
#[derive(Default)]
pub(crate) struct Systems {
    systems: Vec<Box<dyn System>>
}

impl Systems {
    pub fn register<Q: QueryComponents + 'static, F: QueryFilters + 'static, S: Fn(Query<Q, F>) + 'static>(&mut self, callable: S) {
        let system = if Query::<Q, F>::exclusive() {
            ExclusiveSystem {
                callable,
                _marker: PhantomData
            }.into_system()
        } else {
            SharedSystem {
                callable,
                _marker: PhantomData
            }.into_system()
        };

        self.systems.push(system);
    }
}

impl Deref for Systems {
    type Target = Vec<Box<dyn System>>;

    fn deref(&self) -> &Self::Target {
        &self.systems
    }
}

impl DerefMut for Systems {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.systems
    }
}