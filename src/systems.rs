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

pub struct CMDRenderer{
    buffer: Vec<char>,
    size: (usize, usize)
}
impl System for CMDRenderer{
    type Data<'a> = ();
    const ID: &'static str = "CMDRenderer";
    const TYPE: SystemType = SystemType::Postprocessor;

    fn new() -> Self {
        Self{
            buffer: Vec::new(),
            size: (0, 0)
        }
    }

    fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
        use crossterm::{cursor, style, terminal};
        use crossterm::{execute, queue};
        use std::io::{stdout, Write};

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        let size = match terminal::size(){
            Ok(size) => {
                eprint!("DEBUG: Terminal size: {:?}", size);
                (size.0 as usize, size.1 as usize)
            },
            Err(_) => {
                eprint!("DEBUG: Couldn't get Terminal size. Defaulting to (16, 9). Resize your terminal accordingly");
                (16, 9)
            },
        };

        // Here to prevent unnecessary memory changes
        if self.size != size{
            self.buffer = vec![' '; size.0 * size.1];
        }

        // Corner markings
        // X + Y * sizeX
        self.buffer[0 + 0 * self.size.0 ] = '#';
        self.buffer[(self.size.0 - 1) + 0 * self.size.0 ] = '#';
        self.buffer[0 + (self.size.1 - 1) * self.size.0 ] = '#';
        self.buffer[(self.size.0 - 1) + (self.size.1 - 1) * self.size.0] = '#';

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        for line in self.buffer.chunks(self.size.0){
            for chr in line.iter(){
                queue!(stdout(), style::Print(chr)).ok();
            }
            stdout().flush().ok();
        };
    }
}