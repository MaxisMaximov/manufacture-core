use super::*;
use comp::*;
use resources::*;
use events::*;
use commands::*;
pub struct CMDInputHandler;
impl System for CMDInputHandler{
    type Data = &'static mut CMDInput;

    const ID: &'static str = "CMDInput";

    fn new() -> Self {
        Self
    }

    fn execute(&mut self, mut Data: Request<'_, Self::Data>) {
        use crossterm::event::{Event, read, poll};
        if poll(std::time::Duration::from_millis(0)).unwrap(){
            if let Event::Key(key) = read().unwrap(){
                // Triple Deref, whoops
                ***Data = key
            }
        }else{
            ***Data = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
        }
    }
}