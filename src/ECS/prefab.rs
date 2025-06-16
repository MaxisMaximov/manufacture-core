use std::any::Any;

use super::builders::EntityBuilder;

pub trait gmPrefab: Any{
    fn spawn(&self, IN_builder: EntityBuilder);
}