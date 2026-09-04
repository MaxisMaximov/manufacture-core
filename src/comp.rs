use std::collections::HashSet;

use super::*;
use super::storage::*;
use super::types::{Vector2, Vector3};

/// 2D Transform Component
/// 
/// Holds XY position, head-on rotation in Radians and XY scale
pub struct Transform2D{
    pub loc: Vector2,
    pub rot: f32,
    pub scale: Vector2
}
impl Component for Transform2D{
    type STORAGE = BTreeMapStorage<Self>;

    const ID: &'static str = "Transform2D";
}

/// 3D Transform Component
/// 
/// Holds position in all 3 axis, XYZ Euler rotation in Radians and scale in all 3 axis
/// 
/// Note: Z is up in this engine
pub struct Transform3D{
    pub loc: Vector3,
    pub rot: Vector3,
    pub scale: Vector3
}
impl Component for Transform3D{
    type STORAGE = BTreeMapStorage<Self>;

    const ID: &'static str = "Transform3D";
}

/// Holds tags for a given Entity
pub struct Tags{
    inner: HashSet<&'static str>
}
impl Tags{
    pub fn new() -> Self{
        Self{
            inner: HashSet::new(),
        }
    }
    /// Check if this Entity has a given tag
    pub fn has<T: Tag>(&self) -> bool{
        self.inner.contains(T::ID)
    }
    /// Check if this Entity has a given tag via it's ID
    /// 
    /// Note: Because this method takes a tag ID it may not line up with the tag from another plugin you're using
    pub fn has_id(&self, tag: &'static str) -> bool{
        self.inner.contains(tag)
    }
    /// Tag this entity with a tag
    pub fn tag<T: Tag>(&mut self){
        self.inner.insert(T::ID);
    }
    /// Tag this entity with a tag
    /// 
    /// Note: Because this method takes a tag ID it may not line up with the tag from another plugin you're using
    pub fn tag_id(&mut self, tag: &'static str){
        self.inner.insert(tag);
    }
    /// Remove the given tag from the Entity
    pub fn untag<T: Tag>(&mut self){
        self.inner.remove(T::ID);
    }
    /// Remove the given tag from the Entity
    /// 
    /// Note: Because this method takes a tag ID it may not line up with the tag from another plugin you're using
    pub fn untag_id(&mut self, tag: &'static str){
        self.inner.remove(tag);
    }
}
impl Component for Tags{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "Tags";
}
/// Tag trait
/// 
/// Rudimentary trait for ease of use of `Tags` component
pub trait Tag{
    const ID: &'static str;
}

/// A Command-Line sprite
/// 
/// Represents an Entity that can be drawn by CMDRenderer
/// 
/// Origin is at top left, represented by `(0, 0)`
/// 
/// TODO: Add varying origin point
pub struct CMDSprite{
    pub id: String,
    pub z_index: u16,
}
impl Component for CMDSprite{
    type STORAGE = BTreeMapStorage<Self>;

    const ID: &'static str = "CMDSprite";
}

/// Identifies an Entity as being controlled by the player
/// 
/// Typically used to direct player actions to a specific entity
pub struct PlayerController{
    pub pid: u32,
    pub active: bool,
}
impl Component for PlayerController{
    type STORAGE = VecStorage<Self>;

    const ID: &'static str = "PlayerController";
}