use super::storage::gmStorage;
use std::any::Any;

/// # Component trait
/// A trait identifying Components within the engine
/// 
/// `Storage` is anything implementing `gmStorage` trait
/// 
/// `ID` is what the component will be identified by in the World
/// 
/// ## WARNING
/// Make sure your Component ID does not collide with other IDs from other plugins
pub trait Component: Any + Sized{
    type STORAGE: gmStorage<Self>;
    const ID: &'static str;
}