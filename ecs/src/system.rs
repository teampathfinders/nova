use crate::{ComponentQuery, QueryFilter, Query};

pub trait System {}

impl<Q: ComponentQuery, F: QueryFilter> System for for<'a> fn(Query<'a, Q, F>) {}
impl<Q: ComponentQuery, F: QueryFilter> System for SimpleSystem<Q, F> {}

pub struct SimpleSystem<Q: ComponentQuery, F: QueryFilter> {
    func: Box<dyn FnMut(Query<Q, F>)>
}

pub trait IntoSystem<Q: ComponentQuery, F: QueryFilter> {
    type System: System;

    fn into_system(this: Self) -> Box<dyn System>;
}

impl<Q: ComponentQuery + 'static, F: QueryFilter + 'static> IntoSystem<Q, F> for SimpleSystem<Q, F> {
    type System = SimpleSystem<Q, F>;

    fn into_system(this: Self) -> Box<dyn System> {
        Box::new(this)
    }
}

pub struct Systems {
    systems: Vec<Box<dyn System>>
}

impl Systems {
    pub fn register<Q: ComponentQuery + 'static, F: QueryFilter + 'static, S: FnMut(Query<Q, F>) + 'static>(&mut self, system: S) {
        let simple = SimpleSystem {
            func: Box::new(system)
        };

        let system = SimpleSystem::into_system(simple);
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