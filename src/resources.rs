use std::collections::HashMap;

use super::*;
use types::*;

// -- Re-exports --
pub use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

const CMD_KEY_DEFAULT: KeyEvent = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);

/// # Command Line Terminal data
/// Stores data about the Terminal, 
/// such as key pressed, current size and whether the screen can resize
/// 
/// See `crossterm`'s `KeyEvent` for more
/// 
/// TODO: Remove dependency on Crossterm
pub struct CMDData{
    key: KeyEvent,
    size: (usize, usize),
    can_resize: bool
}
impl CMDData{
    /// Get the current key
    pub fn get_key(&self) -> KeyEvent {
        self.key
    }
    pub fn get_size(&self) -> (usize, usize){
        self.size
    }
    pub fn can_resize(&self) -> bool{
        self.can_resize
    }
    /// Set the current key
    pub(crate) fn set_key(&mut self, key: KeyEvent){
        self.key = key
    }
    pub(crate) fn set_size(&mut self, size: (usize, usize)){
        self.size = size
    }
    /// Set key back to Null
    pub(crate) fn reset_key(&mut self){
        self.key = CMD_KEY_DEFAULT
    }
    pub(crate) fn set_resize(&mut self, b: bool){
        self.can_resize = b
    }
}
impl Resource for CMDData{
    const ID: &'static str = "CMDInputData";

    fn new() -> Self {
        Self{
            key: CMD_KEY_DEFAULT,
            size: systems::CMD_SIZE_DEFAULT,
            can_resize: true
        }
    }
}

/// # Command Line Renderer Camera
/// Hold the position of the camera for `CMDRenderer`
/// 
/// Note: Multiple cameras are unsupported right now
/// 
/// Note: Currently the camera can only point downwards pointing towards +Y
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

/// # Command Line Renderer Sprite Registry
/// Holds sprites scheduled to be drawn by `CMDRenderCommand`
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
    /// Register a sprite under ID
    pub fn register(&mut self, id: String, sprite: ASCIIImage){
        self.inner.insert(id, sprite);
    }
    /// Unregister a sprite
    pub fn unregister(&mut self, id: &'static str){
        self.inner.remove(id);
    }
    /// Get an immutable reference to a sprite
    pub fn get<'a>(&'a self, id: &'a str) -> Option<&'a ASCIIImage>{
        self.inner.get(id)
    }
    /// Get a mutable reference to a sprite
    pub fn get_mut<'a>(&'a mut self, id: &'a str) -> Option<&'a mut ASCIIImage>{
        self.inner.get_mut(id)
    }
}

/// Minimum size for CMD Render Queue
const CMD_QUEUE_DEFAULT: usize = 32;

/// # Command Line Render Queue
/// Holds commands for `CMDRenderer` to execute
pub struct CMDRenderQueue{
    inner: Vec<CMDRenderCommand>
}
impl Resource for CMDRenderQueue{
    const ID: &'static str = "CMDRendererQueue";

    fn new() -> Self {
        Self{
            inner: Vec::with_capacity(CMD_QUEUE_DEFAULT),
        }
    }
}
impl CMDRenderQueue{
    /// Push a new command into the queue
    pub fn push(&mut self, command: CMDRenderCommand){
        self.inner.push(command);
    }
    /// Iterate over all commands in the queue
    pub fn iter(&self) -> std::slice::Iter<'_, CMDRenderCommand>{
        self.inner.iter()
    }
    /// Clear the queue
    /// 
    /// **WARNING**: Clearing the queue may cause important things to not render
    pub fn clear(&mut self){
        self.inner.clear();
    }
}