use super::*;

use comp::Component;
use storage::Storage;
use world::gmWorld;

/// # Entity Builder
/// A safe and easy way to contruct a new Entity in the World
#[must_use]
pub struct EntityBuilder<'a>{
    pub(super) entity: usize,
    pub(super) world_ref: &'a mut gmWorld
}
impl<'a> EntityBuilder<'a>{
    /// Add a specified component to the current Entity
    pub fn with<T: Component>(self, Comp: T) -> Self{
        self.world_ref.fetchMut::<T>().insert(self.entity, Comp);
        self
    }

    pub fn finish(self){}
}