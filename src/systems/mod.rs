#[cfg(feature = "cmd_render_test")]
use std::time::Instant;

use super::*;
use resources::*;
use types::*;

mod cmd_render;
pub use cmd_render::*;

/// # Command Line Data Getter
/// Acquires data from the Command Line terminal:
/// - Current pressed key from the Command Line in Raw Mode
/// - Current Terminal screen size
/// - Whether the screen can resize
/// 
/// Note: Some terminals may put `Press` and `Hold` events
/// at the same time when you press a key in Raw Mode
/// 
/// Note: Holding a key in Raw Mode floods the input buffer
/// and may prevent the Getter from reading other keys for a while
/// 
/// TODO: Fix the double input issue
pub struct CMDDataGetter;
impl System for CMDDataGetter{
    type Data<'a> = &'a mut CMDData;
    const ID: &'static str = "CMDInput";
    const TYPE: SystemType = SystemType::Preprocessor;
    
    fn new() -> Self { Self }
    
    fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
        use crossterm::{event::{Event, read, poll}, terminal};

        if poll(std::time::Duration::from_millis(0)).unwrap(){
            if let Event::Key(key) = read().unwrap(){
                data.set_key(key)
            }
        }else{
            data.reset_key();
        }

        if data.can_resize(){
            match terminal::size(){
                Ok(size) => {
                    data.set_size((size.0 as usize, size.1 as usize));
                },
                Err(_) => {
                    eprint!("ERROR: Couldn't get Terminal size. Defaulting to {:?}. Resize your terminal accordingly", CMD_SIZE_DEFAULT);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    data.set_resize(false);
                    data.set_size(CMD_SIZE_DEFAULT);
                },
            };
        }
    }
}