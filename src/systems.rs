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
type CMDColor = (u8, u8, u8);
const CMD_FG_DEFAULT: CMDColor = (255, 255, 255);
const CMD_BG_DEFAULT: CMDColor = (0, 0, 0);

pub struct CMDRenderer{
    buffer: Vec<(char, CMDColor, CMDColor)>,
    size: CMDCoords,
}

impl System for CMDRenderer{
    type Data<'a> = ();
    const ID: &'static str = "CMDRenderer";
    const TYPE: SystemType = SystemType::Postprocessor;

    fn new() -> Self {
        Self{
            buffer: Vec::new(),
            size: (100, 20),
        }
    }

    fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
        use crossterm::{cursor, style::{self, Stylize}, terminal};
        use crossterm::{execute, queue};
        use std::io::{stdout, Write};

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        let cmd_size = match terminal::size(){
            Ok(size) => {
                (size.0 as usize, size.1 as usize)
            },
            Err(_) => {
                eprint!("DEBUG: Couldn't get Terminal size. Defaulting to (32, 18). Resize your terminal accordingly");
                std::thread::sleep(std::time::Duration::from_secs(5));
                (100, 20)
            },
        };

        // Here to prevent unnecessary memory changes
        if self.size != cmd_size{
            self.buffer = vec![(' ', CMD_FG_DEFAULT, CMD_BG_DEFAULT); cmd_size.0 * cmd_size.1];
            self.size = cmd_size;
        }

        // Criss/cross lines
        self.draw_line((0, 0), (self.size.0-1, self.size.1-1), '■', (255, 0, 0), CMD_BG_DEFAULT);
        self.draw_line((0, self.size.1-1), (self.size.0-1, 0), '■', (255, 0, 0), CMD_BG_DEFAULT);

        // Corner markings
        self.plot(0, 0, '#', (255, 0, 0), CMD_BG_DEFAULT);
        self.plot(self.size.0 - 1, 0, '#', (255, 0, 0), CMD_BG_DEFAULT);
        self.plot(0, self.size.1 - 1, '#', (255, 0, 0), CMD_BG_DEFAULT);
        self.plot(self.size.0 - 1, self.size.1 - 1, '#', (255, 0, 0), CMD_BG_DEFAULT);

        // Middle Boxes
        {
            let third = (self.size.0 / 3, self.size.1 / 3);
            self.draw_rect(third, (self.size.0 - third.0, self.size.1 - third.1), '#', CMD_FG_DEFAULT, (0, 0, 255));

            self.draw_box((third.0 - 2, third.1 - 2), (self.size.0 - third.0 + 2, self.size.1 - third.1 + 2), '=', CMD_FG_DEFAULT, (0, 0, 255));
        }

        // Boundary border
        self.draw_box((1, 1), (self.size.0 - 2, self.size.1 - 2), '#', CMD_FG_DEFAULT, CMD_BG_DEFAULT);

        // Debug text
        self.write_text((3, 3), &format!("DEBUG: Terminal size: {:?}", self.size), CMD_FG_DEFAULT, CMD_BG_DEFAULT);
        self.draw_sequence(
            (self.size.0 / 2, self.size.1 / 2), 
            &[
                ('H', (255, 0, 0), (0, 255, 255)),
                ('e', (255, 128, 0), (0, 128, 255)),
                ('l', (255, 255, 0), (0, 0, 255)),
                ('l', (128, 255, 0), (128, 0, 255)),
                ('o', (0, 255, 0), (255, 0, 255)),
                (' ', CMD_FG_DEFAULT, (255, 0, 128)),
                ('W', (0, 255, 255), (255, 0, 0)),
                ('o', (0, 128, 255), (255, 128, 0)),
                ('r', (0, 0, 255), (255, 255, 0)),
                ('l', (128, 0, 255), (128, 255, 0)),
                ('d', (255, 0, 255), (0, 255, 0)),
                ]
        );

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        for line in self.buffer.chunks(self.size.0){
            for (chr, fg, bg) in line.iter(){
                queue!(stdout(), 
                    style::Print(
                        chr.with(style::Color::Rgb{
                            r: fg.0,
                            g: fg.1,
                            b: fg.2
                        })
                        .on(style::Color::Rgb{
                            r: bg.0,
                            g: bg.1,
                            b: bg.2
                        })
                    )
                ).ok();
            }
            stdout().flush().ok();
        };
    }
}
impl CMDRenderer{
    #[inline(always)]
    fn plot(&mut self, x: usize, y: usize, chr: char, fg: CMDColor, bg: CMDColor){
        if (x, y) > self.size{ return }
        self.buffer[x + y*self.size.0] = (chr, fg, bg);
    }
    /// Uses Brehensam algorithm modified to work purely on unsigned integers
    fn draw_line(&mut self, a: CMDCoords, b: CMDCoords, chr: char, fg: CMDColor, bg: CMDColor){
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
                self.plot(x, y, chr, fg, bg);

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
                self.plot(x, y, chr, fg, bg);

                err -= delta_x;

                if err <= delta_x{
                    err += delta_y;
                    if start.0 < end.0{ x += 1 }else{ x -= 1 }
                }
            }
        }
    }
    fn write_text(&mut self, pos: CMDCoords, text: &str, fg: CMDColor, bg: CMDColor){
        for (offset, chr) in text.char_indices(){
            self.plot(pos.0 + offset, pos.1, chr, fg, bg);
        }
    }
    fn draw_sequence(&mut self, pos: CMDCoords, sequence: &[(char, CMDColor, CMDColor)]){
        for (offset, (chr, fg, bg)) in sequence.iter().enumerate(){
            self.plot(pos.0 + offset, pos.1, *chr, *fg, *bg);
        }
    }
    fn draw_rect(&mut self, a: CMDCoords, b: CMDCoords, chr: char, fg: CMDColor, bg: CMDColor){
        let (tr, bl) = if a < b { (a, b) }else{ (b, a) };

        for x in tr.0..=bl.0{
            for y in tr.1..=bl.1{
                self.plot(x, y, chr, fg, bg);
            }
        }
    }
    fn draw_box(&mut self, a: CMDCoords, b: CMDCoords, chr: char, fg: CMDColor, bg: CMDColor){
        let (tr, bl) = if a < b { (a, b) }else{ (b, a) };

        for y in [tr.1, bl.1]{
            for x in tr.0..=bl.0{
                self.plot(x, y, chr, fg, bg);
            }
        }
        for x in [tr.0, bl.0]{
            for y in tr.1..=bl.1{
                self.plot(x, y, chr, fg, bg);
            }
        }
    }
}