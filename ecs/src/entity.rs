pub trait Bundle {
    fn get(&self) -> Vec<&dyn Component>;
}

impl<T: Component> Bundle for T {
    fn get(&self) -> Vec<&dyn Component> {
        vec![self]
    }
}

impl<T: Component> Bundle for [T] {
    fn get(&self) -> Vec<&dyn Component> {
        self.iter().map(|c| c as &dyn Component).collect::<Vec<_>>()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Entity(usize);

impl From<usize> for Entity {
    fn from(id: usize) -> Self {
        Self(id)
    } 
}

/// Holds a list of all components of a type.
/// This is done to improve cache locality, all similar components are put next to each other.
pub(crate) struct ComponentPack<T: Component> {
    components: Vec<T>
}

pub(crate) struct EntityComponents {
    // TODO: This could probably be done using a vector instead of a map.
    // components: Vec<>
}

pub trait Component {}