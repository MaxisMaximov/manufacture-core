use super::*;

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
    pub fn set(&mut self, key: KeyEvent){
        self.key = key
    }
    /// Set key back to Null
    pub fn reset(&mut self){
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