use crate::{comp::{CMDSprite, Transform2D, Transform3D}};

use super::*;
use resources::*;

/// # Command Line Input Handler
/// Acquires the current pressed key from the Command Line
/// 
/// Note: Some terminals may put `Press` and `Hold` events
/// at the same timewhen you press a key
/// 
/// Note: Holding a key in Raw Mode floods the input buffer
/// and may prevent the Handler from reading other keys
/// 
/// TODO: Fix the double input issue
pub struct CMDInputGetter;
impl System for CMDInputGetter{
    type Data<'a> = &'a mut CMDInput;
    const ID: &'static str = "CMDInput";
    const TYPE: SystemType = SystemType::Preprocessor;

    fn new() -> Self { Self }

    fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
        use crossterm::event::{Event, read, poll};
        if poll(std::time::Duration::from_millis(0)).unwrap(){
            if let Event::Key(key) = read().unwrap(){
                data.set(key)
            }
        }else{
            data.reset();
        }
    }
}

pub struct CMDRenderer;
impl System for CMDRenderer{
    type Data<'a> = Query<(&'a CMDSprite, Option<&'a Transform2D>, Option<&'a Transform3D>), ()>;
    const ID: &'static str = "CMDRenderer";
    const TYPE: SystemType = SystemType::Postprocessor;

    fn new() -> Self {
        Self
    }

    fn execute(&mut self, data: Request<'_, Self::Data<'_>>) {
        todo!()
    }
}