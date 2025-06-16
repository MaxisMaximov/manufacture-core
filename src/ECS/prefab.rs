use std::any::Any;

use super::entity::EntityBuilder;

pub trait gmPrefab: Any{
    fn spawn(&self, IN_builder: EntityBuilder);
}