use crate::comp::{CMDSprite, Transform2D, Transform3D};

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
    type Data<'a> = ();
    const ID: &'static str = "CMDRenderer";
    const TYPE: SystemType = SystemType::Postprocessor;

    fn new() -> Self {
        Self
    }

    fn execute(&mut self, data: Request<'_, Self::Data<'_>>) {
        use crossterm::{cursor, style, terminal};
        use crossterm::execute;
        use std::io::{stdout, Write};

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        let size = match terminal::size(){
            Ok(size) => {
                eprint!("DEBUG: Terminal size: {:?}", size);
                size
            },
            Err(_) => {
                eprint!("DEBUG: Couldn't get Terminal size. Defaulting to (16, 9). Resize your terminal accordingly");
                (16, 9)
            },
        };

        let mut buffer = vec![' '; size.0 as usize * size.1 as usize];

        // Corner markings
        // X + Y * sizeX
        buffer[(0 + 0 * size.0) as usize] = '#';
        buffer[((size.0 - 1) + 0 * size.0) as usize] = '#';
        buffer[(0 + (size.1 - 1) * size.0) as usize] = '#';
        buffer[((size.0 - 1) + (size.1 - 1) * size.0) as usize] = '#';

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        for line in buffer.chunks(size.0 as usize){
            for chr in line.iter(){
                print!("{}", chr);
            }
            execute!(stdout(), cursor::MoveToNextLine(1)).ok();
        };

        stdout().flush().ok();
    }
}