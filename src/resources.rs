use std::collections::HashMap;

use super::*;
use types::*;

pub use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
/// # User input -- CMD
/// Stores the input provided by the Command Line
/// 
/// See `crossterm`'s `KeyEvent` for more
/// 
/// TODO: Remove dependency on Crossterm
pub struct CMDInput{
    key: KeyEvent
}
impl CMDInput{
    /// Get the current key
    pub fn get(&self) -> KeyEvent {
        self.key
    }
    /// Set the current key
    pub(crate) fn set(&mut self, key: KeyEvent){
        self.key = key
    }
    /// Set key back to Null
    pub(crate) fn reset(&mut self){
        self.key = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
    }
}
impl Resource for CMDInput{
    const ID: &'static str = "CMDInputData";

    fn new() -> Self {
        Self{
            key: KeyEvent::new(KeyCode::Null, KeyModifiers::NONE),
        }
    }
}

/// # CMDRenderer Camera
/// Hold the position of the camera for `CMDRenderer`
/// 
/// Note: Multiple cameras are unsupported right now
pub struct CMDCamera{
    pub pos: Vector2
}
impl Resource for CMDCamera{
    const ID: &'static str = "CMDCamera";

    fn new() -> Self {
        Self{
            pos: Vector2 { x: 0.0, y: 0.0 },
        }
    }
}

pub struct CMDSpriteRegistry{
    inner: HashMap<&'static str, ASCIIImage>
}
impl Resource for CMDSpriteRegistry{
    const ID: &'static str = "CMDSpriteRegistry";

    fn new() -> Self {
        Self{
            inner: HashMap::new(),
        }
    }
}
impl CMDSpriteRegistry{
    pub fn register(&mut self, id: &'static str, sprite: ASCIIImage){
        self.inner.insert(id, sprite);
    }
    pub fn unregister(&mut self, id: &'static str){
        self.inner.remove(id);
    }
    pub fn get(&self, id: &'static str) -> Option<&ASCIIImage>{
        self.inner.get(id)
    }
    pub fn get_mut(&mut self, id: &'static str) -> Option<&mut ASCIIImage>{
        self.inner.get_mut(id)
    }
}