use std::{any::TypeId, collections::HashMap};

pub trait Component {}

pub struct ComponentInfo {
    type_id: TypeId
}

pub struct ComponentStorage<T: Component> {
    storage: Vec<T>    
}

pub trait Storage {
    fn type_id(&self) -> TypeId;
}

impl<T: Component + 'static> Storage for ComponentStorage<T> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

pub struct Components {
    storage: HashMap<TypeId, Box<dyn Storage>>
}

impl Default for Components {
    fn default() -> Components {
        Components {
            storage: HashMap::new()
        }
    }
}

pub trait Collection {
    fn insert(&self, callback: &mut impl FnMut(TypeId));
}

impl<C: Component + 'static> Collection for C {
    fn insert(&self, callback: &mut impl FnMut(TypeId)) {
        callback(TypeId::of::<C>())
    }
}