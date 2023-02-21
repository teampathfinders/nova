use std::{collections::HashMap, any::TypeId};

use crate::{ComponentQuery, QueryFilter, Query};

pub trait System {

}

pub struct Systems {
    
}

impl Systems {
    pub fn register<Q: ComponentQuery, F: QueryFilter, S: FnMut(Query<Q, F>)>(&mut self, system: S) {
        
    }
}

impl Default for Systems {
    fn default() -> Systems {
        Systems {}
    }
}