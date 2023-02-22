use std::marker::PhantomData;

use crate::{QueryComponents, QueryFilters, Query, QueryDescriptor};

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
    /// A descriptor of the query that this system has requested.
    fn query(&self) -> QueryDescriptor;
    /// The system variant.
    /// This can be ether of the values of [`SystemVariant`].
    fn variant(&self) -> SystemVariant;
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

    fn query(&self) -> QueryDescriptor {
        Query::<Q, F>::descriptor()
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

    fn query(&self) -> QueryDescriptor {
        Query::<Q, F>::descriptor()
    }
}

impl<Q: QueryComponents + 'static, F: QueryFilters + 'static, S: FnMut(Query<Q, F>) + 'static> IntoSystem for ExclusiveSystem<Q, F, S> {
    fn into_system(self) -> Box<dyn System> {
        Box::new(self)
    }
}

/// Keeps track of the currently active systems.
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

    pub fn call_all(&self) {
        self.systems.iter().for_each(|s| {
            let meta = s.query();
            let variant = s.variant();

            println!("System is {variant:?} with {meta:#?}");
        });
    }
}

impl Default for Systems {
    fn default() -> Systems {
        Systems {
            systems: Vec::new()
        }
    }
}