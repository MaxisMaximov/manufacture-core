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
    inner: HashMap<String, ASCIIImage>
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
    pub fn register(&mut self, id: String, sprite: ASCIIImage){
        self.inner.insert(id, sprite);
    }
    pub fn unregister(&mut self, id: &'static str){
        self.inner.remove(id);
    }
    pub fn get<'a>(&'a self, id: &'a str) -> Option<&'a ASCIIImage>{
        self.inner.get(id)
    }
    pub fn get_mut<'a>(&'a mut self, id: &'a str) -> Option<&'a mut ASCIIImage>{
        self.inner.get_mut(id)
    }
}

const CMD_QUEUE_DEFAULT: usize = 32;

pub struct CMDRendererQueue{
    inner: Vec<CMDRenderCommand>
}
impl Resource for CMDRendererQueue{
    const ID: &'static str = "CMDRendererQueue";

    fn new() -> Self {
        Self{
            inner: Vec::with_capacity(CMD_QUEUE_DEFAULT),
        }
    }
}
impl CMDRendererQueue{
    pub fn push(&mut self, command: CMDRenderCommand){
        self.inner.push(command);
    }
    pub fn iter(&self) -> std::slice::Iter<'_, CMDRenderCommand>{
        self.inner.iter()
    }
    pub fn clear(&mut self){
        self.inner.clear();
    }
}