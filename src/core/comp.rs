use std::collections::HashSet;

use super::*;
use crate::core::storage::*;
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
    pub fn has(&self, tag: &'static str) -> bool{
        self.inner.contains(tag)
    }
    /// Tag this entity with a tag
    pub fn tag(&mut self, tag: &'static str){
        self.inner.insert(tag);
    }
    /// Remove the given tag from the Entity
    pub fn untag(&mut self, tag: &'static str){
        self.inner.remove(tag);
    }
}
impl Component for Tags{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "Tags";
}

/// A Command-Line sprite
/// 
/// Represents a 2D ASCII art image
/// 
/// Individual *"pixels"* are `(ch, fg, bg)` tuples: `ch`aracter, `f`ore`g`round color and `b`ack`g`round color.  
/// FG and BG colors are `(R, G, B)` tuples that use `u8` as values
pub struct CMDSprite{
    pub size_x: u8,
    pub size_y: u8,
    pub z_index: u16,
    pub data: Vec<(char, (u8, u8, u8), (u8, u8, u8))> // Symbol, Foreground RGB, Background RGB
}
impl Component for CMDSprite{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "CMDSprite";
}

/// Identifies an Entity as being controlled by the player
/// 
/// Typically used to direct player actions to a specific entity
pub struct PlayerController{
    pub pid: u16,
    pub active: bool,
}
impl Component for PlayerController{
    type STORAGE = VecStorage<Self>;

    const ID: &'static str = "PlayerController";
}