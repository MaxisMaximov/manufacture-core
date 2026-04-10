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

type CMDCoords = (usize, usize);

pub struct CMDRenderer{
    buffer: Vec<char>,
    size: CMDCoords
}
impl System for CMDRenderer{
    type Data<'a> = ();
    const ID: &'static str = "CMDRenderer";
    const TYPE: SystemType = SystemType::Postprocessor;

    fn new() -> Self {
        Self{
            buffer: Vec::new(),
            size: (100, 20)
        }
    }

    fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
        use crossterm::{cursor, style, terminal};
        use crossterm::{execute, queue};
        use std::io::{stdout, Write};

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        let cmd_size = match terminal::size(){
            Ok(size) => {
                eprint!("DEBUG: Terminal size: {:?}", size);
                (size.0 as usize, size.1 as usize)
            },
            Err(_) => {
                eprint!("DEBUG: Couldn't get Terminal size. Defaulting to (32, 18). Resize your terminal accordingly");
                (100, 20)
            },
        };

        // Here to prevent unnecessary memory changes
        if self.size != cmd_size{
            self.buffer = vec![' '; cmd_size.0 * cmd_size.1];
            self.size = cmd_size;
        }

        // Criss/cross lines
        self.draw_line((0, 0), (self.size.0-1, self.size.1-1), '■');
        self.draw_line((0, self.size.1-1), (self.size.0-1, 0), '■');

        // Corner markings
        self.plot(0, 0, '#');
        self.plot(self.size.0 - 1, 0, '#');
        self.plot(0, self.size.1 - 1, '#');
        self.plot(self.size.0 - 1, self.size.1 - 1, '#');


        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        for line in self.buffer.chunks(self.size.0){
            for chr in line.iter(){
                queue!(stdout(), style::Print(chr)).ok();
            }
            stdout().flush().ok();
        };
    }
}
impl CMDRenderer{
    fn plot(&mut self, x: usize, y: usize, chr: char){
        self.buffer[x + y*self.size.0] = chr;
    }
    /// Uses Brehensam algorithm modified to work purely on unsigned integers
    fn draw_line(&mut self, a: CMDCoords, b: CMDCoords, chr: char){
        let delta_x = a.0.abs_diff(b.0);
        let delta_y = a.1.abs_diff(b.1);

        if delta_x >= delta_y{

            let (start, end) = {
                // Swap A and B if B is closer to (0, 0)
                if a.0 < b.0{ (a, b) }else{ (b, a) }
            };

            let mut err = delta_x - delta_y;

            let mut y = start.1;

            for x in start.0..=end.0{
                self.plot(x, y, chr);

                err -= delta_y;

                if err <= delta_y{
                    err += delta_x;
                    if start.1 < end.1{ y += 1 }else{ y -= 1 }
                }
            }

        }else{
            let (start, end) = {
                // Swap A and B if B is closer to (0, 0)
                if a.1 < b.1{ (a, b) }else{ (b, a) }
            };

            let mut err = delta_y - delta_x;

            let mut x = start.0;

            for y in start.1..=end.1{
                self.plot(x, y, chr);

                err -= delta_x;

                if err <= delta_x{
                    err += delta_y;
                    if start.0 < end.0{ x += 1 }else{ x -= 1 }
                }
            }
        }
    }
}