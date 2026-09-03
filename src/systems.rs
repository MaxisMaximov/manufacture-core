use std::time::Instant;

use super::*;
use resources::*;
use types::*;

/// # Command Line Input Handler
/// Acquires the current pressed key from the Command Line
/// 
/// Note: Some terminals may put `Press` and `Hold` events
/// at the same time when you press a key
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

const CMD_CHR_DEFAULT: char = ' ';
const CMD_FG_DEFAULT: CMDColor = (255, 255, 255);
const CMD_BG_DEFAULT: CMDColor = (0, 0, 0);
const CMD_CELL_DEFAULT: (char, CMDColor, CMDColor) = (CMD_CHR_DEFAULT, CMD_FG_DEFAULT, CMD_BG_DEFAULT);
const CMD_SIZE_DEFAULT: (usize, usize) = (100, 20);

pub struct CMDRenderer{
    buffer: Vec<(char, CMDColor, CMDColor)>,
    size: (usize, usize),
    
    profiler: CMDRendererProfiler
}

impl System for CMDRenderer{
    type Data<'a> = (&'a DeltaT, &'a mut CMDRendererQueue, &'a CMDSpriteRegistry);
    const ID: &'static str = "CMDRenderer";
    const TYPE: SystemType = SystemType::Postprocessor;

    fn new() -> Self {
        Self{
            buffer: vec![CMD_CELL_DEFAULT; CMD_SIZE_DEFAULT.0 * CMD_SIZE_DEFAULT.1],
            size: CMD_SIZE_DEFAULT,
            
            profiler: CMDRendererProfiler::new()
        }
    }

    fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
        use crossterm::{cursor, style, terminal};
        use crossterm::{execute, queue};
        use std::io::{stdout, Write};

        let (
            delta_t,
            mut render_queue,
            sprite_registry
        ) = _data.into_raw();

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        self.profiler.start_frame_profile();
        let mut lock = stdout().lock();

        let cmd_size = match terminal::size(){
            Ok(size) => {
                (size.0 as usize, size.1 as usize)
            },
            Err(_) => {
                eprint!("DEBUG: Couldn't get Terminal size. Defaulting to {:?}. Resize your terminal accordingly", CMD_SIZE_DEFAULT);
                std::thread::sleep(std::time::Duration::from_secs(5));
                CMD_SIZE_DEFAULT
            },
        };

        // Here to prevent unnecessary memory changes
        if self.size != cmd_size{
            self.buffer.resize(cmd_size.0 * cmd_size.1, CMD_CELL_DEFAULT);
            self.size = cmd_size;
        }

        self.clear_buffer();

        for cmd in render_queue.iter(){
            match cmd{
                CMDRenderCommand::DrawLine { a, b, chr, fg, bg } => {
                    let a = self.ndc_to_ss(*a);
                    let b = self.ndc_to_ss(*b);
                    self.draw_line(a, b, *chr, *fg, *bg)
                },
                CMDRenderCommand::WriteText { pos, text, fg, bg } => self.write_text_ndc(*pos, text, *fg, *bg),
                CMDRenderCommand::DrawSequence { pos, sequence } => self.draw_sequence(*pos, sequence),
                CMDRenderCommand::DrawRect { a, b, chr, fg, bg } => self.draw_rect_ndc(*a, *b, *chr, *fg, *bg),
                CMDRenderCommand::DrawBox { a, b, chr, fg, bg } => self.draw_box_ndc(*a, *b, *chr, *fg, *bg),
                CMDRenderCommand::DrawSprite { pos, sprite_id } => {
                    if let Some(sprite) = sprite_registry.get(sprite_id) {
                        self.draw_sprite_ndc(*pos, sprite)
                    }
                },
            }
        }
        render_queue.clear();

        // -- DEBUG RENDERS --

        // Criss/cross lines
        {
            let bl = self.ndc_to_ss((-1.0, -1.0));
            let tr = self.ndc_to_ss((1.0, 1.0));
            self.draw_line((bl.0, tr.1), (tr.0, bl.1), '■', (255, 0, 0), CMD_BG_DEFAULT);
            self.draw_line(bl, tr, '■', (255, 0, 0), CMD_BG_DEFAULT);

            // Corner markings
            self.plot_px(bl.0, tr.1, '#', (255, 0, 0), CMD_BG_DEFAULT);
            self.plot_px(tr.0, bl.1, '#', (255, 0, 0), CMD_BG_DEFAULT);
            self.plot_px(bl.0, bl.1, '#', (255, 0, 0), CMD_BG_DEFAULT);
            self.plot_px(tr.0, tr.1, '#', (255, 0, 0), CMD_BG_DEFAULT);
        }


        // Middle Boxes
        {
            self.draw_rect_ndc((-0.333, -0.333), (0.333, 0.333), '#', CMD_FG_DEFAULT, (0, 0, 255));

            self.draw_box_ndc((-0.4, -0.4), (0.4, 0.4), '=', CMD_FG_DEFAULT, (0, 0, 255));
        }

        // Boundary border
        self.draw_box_ndc((-1.0, -1.0), (1.0, 1.0), '#', CMD_FG_DEFAULT, CMD_BG_DEFAULT);

        // Sprite test
        self.draw_sprite_ndc((-1.0, 0.0), &sprite_registry.get("CMD_RENDER_TEST").unwrap()); // UNWRAP: It's 100% guaranteed to be registered in the test build

        self.write_text_ndc(
            (-0.0, 0.0), 
            "Hello\nWorld", 
            CMD_FG_DEFAULT, 
            CMD_BG_DEFAULT
        );

        // Debug Info
        let mut debug_str = String::with_capacity(256);

        debug_str.push_str(&format!("DEBUG: Terminal size: {:?}\n", self.size));


        debug_str.push_str(&format!("DEBUG: Frame: {}; Logic Frame: {}; Last check: {}; Delta: {}\n", delta_t.frame(), delta_t.logic_frame(), self.profiler.last_check_frame, delta_t.frame() - self.profiler.last_check_frame));

        self.profiler.update(delta_t.frame(), delta_t.logic_frame());

        debug_str.push_str(&format!("DEBUG: Estimated FPS: {:?}\n", self.profiler.last_frames));
        
        debug_str.push_str(&format!("DEBUG: Debug frame processing took: {:?}\n", self.profiler.stop_frame_profile()));

        self.write_text_ndc((-0.9, 0.9), &debug_str, CMD_FG_DEFAULT, CMD_BG_DEFAULT);

        // -- RENDER --
        execute!(lock, cursor::MoveTo(0, 0)).ok();

        let mut last = (CMD_FG_DEFAULT, CMD_BG_DEFAULT);
        for (chr, fg, bg) in self.buffer.iter(){
            if *fg != last.0{
                queue!(lock,
                    style::SetForegroundColor(style::Color::Rgb{
                        r: fg.0,
                        g: fg.1,
                        b: fg.2
                    })
                ).ok();
                last.0 = *fg;
            }

            if *bg != last.1{
                queue!(lock,
                    style::SetBackgroundColor(style::Color::Rgb{
                        r: bg.0,
                        g: bg.1,
                        b: bg.2
                    })
                ).ok();
                last.1 = *bg;
            }

            queue!(lock, style::Print(chr)).ok();
        };
        lock.flush().ok();
        drop(lock)
    }
}
impl CMDRenderer{
    fn clear_buffer(&mut self){
        self.buffer.iter_mut().for_each(|cell| *cell = CMD_CELL_DEFAULT);
    }
    #[inline(always)]
    fn plot_px(&mut self, x: isize, y: isize, chr: char, fg: CMDColor, bg: CMDColor){
        // Negative `isize` cast to `usize` is always bigger than 0
        // `self.size` is an EX-clusive range
        if x as usize >= self.size.0 || y as usize >= self.size.1{ return }
        self.buffer[x as usize + y as usize * self.size.0] = (chr, fg, bg);
    }
    #[inline(always)]
    fn ndc_to_ss(&mut self, pos: NDCoords) -> SSCoords{
        // Shift, correct, and fract(?)
        let x = ((pos.0 + 1.0) * 0.5 * (self.size.0-1) as f32) as isize;
        // An additional `* -1` to make -1 = Bottom instead of Top
        let y = ((pos.1 * -1.0 + 1.0) * 0.5 * (self.size.1-1) as f32) as isize;
        (x, y)
    }
    #[deprecated = "Unstable to use, use `inbounds_ndc` instead"]
    fn bounds_check(&self, a: SSCoords, b: SSCoords) -> bool{
        // If either of the coords is inside the bounds, it's fine
        (a.0 as usize, a.1 as usize) < self.size || (b.0 as usize, b.1 as usize) < self.size
    }
    #[inline(always)]
    fn inbounds_ndc(&self, pos: NDCoords) -> bool{
        pos.0 >= 0.0 && pos.1 >= 0.0 && pos.0 <= 1.0 && pos.1 <= 1.0
    }
    /// Uses Brehensam algorithm modified to work purely on unsigned integers
    fn draw_line(&mut self, a: SSCoords, b: SSCoords, chr: char, fg: CMDColor, bg: CMDColor){

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
                self.plot_px(x, y, chr, fg, bg);

                err -= delta_y;

                if err <= delta_y{
                    err += delta_x;
                    if start.1 < end.1{ y += 1 }else{ y -= 1 }
                }
            }

        }else{
            let (start, end) = {
                // Swap A and B if B is closer to (0, 0) Screenspace
                if a.1 < b.1{ (a, b) }else{ (b, a) }
            };

            let mut err = delta_y - delta_x;

            let mut x = start.0;

            for y in start.1..=end.1{
                self.plot_px(x, y, chr, fg, bg);

                err -= delta_x;

                if err <= delta_x{
                    err += delta_y;
                    if start.0 < end.0{ x += 1 }else{ x -= 1 }
                }
            }
        }
    }
    #[deprecated = "Unstable to use, use `write_text_ndc` instead"]
    fn write_text(&mut self, pos: SSCoords, text: &str, fg: CMDColor, bg: CMDColor){
        // We only check the `pos`, all text happens lower down
        if (pos.0 as usize, pos.1 as usize) >= self.size{ return }

        for (y_offset, line) in text.lines().enumerate(){
            for (x_offset, chr) in line.char_indices(){
                self.plot_px(pos.0 + x_offset as isize, pos.1 + y_offset as isize, chr, fg, bg);
            }
        }
    }
    fn write_text_ndc(&mut self, pos: NDCoords, text: &str, fg: CMDColor, bg: CMDColor){
        let origin = self.ndc_to_ss(pos);
        
        for (line_off, line) in text.lines().enumerate(){
            for (chr_off, chr) in line.char_indices(){
                self.plot_px(origin.0 + chr_off as isize, origin.1 + line_off as isize, chr, fg, bg);
            }
        }
    }
    #[deprecated = "Deprecated out of lack of use cases"]
    fn draw_sequence(&mut self, pos: SSCoords, sequence: &[(char, CMDColor, CMDColor)]){
        // We only check the `pos`, all text happens lower down
        if (pos.0 as usize, pos.1 as usize) >= self.size{ return }

        for (x_offset, (chr, fg, bg)) in sequence.iter().enumerate(){
            self.plot_px(pos.0 + x_offset as isize, pos.1, *chr, *fg, *bg);
        }
    }
    #[deprecated = "Unstable to use, use `draw_rect_ndc` instead"]
    fn draw_rect(&mut self, a: SSCoords, b: SSCoords, chr: char, fg: CMDColor, bg: CMDColor){

        if !self.bounds_check(a, b){ return }

        let (tr, bl) = if a < b { (a, b) }else{ (b, a) };

        for x in tr.0..=bl.0{
            for y in tr.1..=bl.1{
                self.plot_px(x, y, chr, fg, bg);
            }
        }
    }
    fn draw_rect_ndc(&mut self, a: NDCoords, b: NDCoords, chr: char, fg: CMDColor, bg: CMDColor){
        if !self.inbounds_ndc(a) && !self.inbounds_ndc(b){ return }
        // ↘↘
        let bl = self.ndc_to_ss((a.0.min(b.0), a.1.min(b.1)));
        let tr = self.ndc_to_ss((a.0.max(b.0), a.1.max(b.1)));
        
        for x in bl.0..=tr.0{
            for y in tr.1..=bl.1{
                self.plot_px(x, y, chr, fg, bg);
            }
        }
    }
    #[deprecated = "Unstable to use, use `draw_box_ndc` instead"]
    fn draw_box(&mut self, a: SSCoords, b: SSCoords, chr: char, fg: CMDColor, bg: CMDColor){

        if !self.bounds_check(a, b){ return }
        
        let (tr, bl) = if a < b { (a, b) }else{ (b, a) };

        for y in [tr.1, bl.1]{
            for x in tr.0..=bl.0{
                self.plot_px(x, y, chr, fg, bg);
            }
        }
        for x in [tr.0, bl.0]{
            for y in tr.1..=bl.1{
                self.plot_px(x, y, chr, fg, bg);
            }
        }
    }
    fn draw_box_ndc(&mut self, a: NDCoords, b: NDCoords, chr: char, fg: CMDColor, bg: CMDColor){
        if !self.inbounds_ndc(a) && !self.inbounds_ndc(b){ return }

        let bl = self.ndc_to_ss((a.0.min(b.0), a.1.min(b.1)));
        let tr = self.ndc_to_ss((a.0.max(b.0), a.1.max(b.1)));

        for x in [bl.0, tr.0]{
            for y in tr.1..=bl.1{
                self.plot_px(x, y, chr, fg, bg);
            }
        }
        for y in [bl.1, tr.1]{
            for x in bl.0..=tr.0{
                self.plot_px(x, y, chr, fg, bg);
            }
        }
    }
    #[deprecated = "Unstable to use, use `draw_sprite_ndc` instead"]
    fn draw_sprite(&mut self, pos: SSCoords, sprite: &types::ASCIIImage){

        if !self.bounds_check(pos, (pos.0 + sprite.size_x as isize, pos.1 + sprite.size_y as isize)){ return }
        
        for (y_offset, row) in sprite.data.chunks(sprite.size_x as usize).enumerate(){
            for (x_offset, (chr, fg, bg)) in row.iter().enumerate(){
                self.plot_px(pos.0 + x_offset as isize, pos.1 + y_offset as isize, *chr, *fg, *bg);
            }
        }
    }
    fn draw_sprite_ndc(&mut self, pos: NDCoords, sprite: &types::ASCIIImage){
        let ss_pos = self.ndc_to_ss(pos);

        for (y_off, row) in sprite.data.chunks(sprite.size_x as usize).enumerate(){
            for (x_off, px) in row.iter().enumerate(){
                self.plot_px(ss_pos.0 + x_off as isize, ss_pos.1 + y_off as isize, px.0, px.1, px.2);
            }
        }
    }
}

/// # CMD Renderer Profiler
/// A rudimentary struct made to neatly contain the profilign information for the renderer
/// 
/// By default it's disabled, compile with Debug flag to enable
/// 
/// TODO: Make it toggleable at runtime
struct CMDRendererProfiler{
    frame_profile_start: Instant,
    last_check_frame: u64,
    last_logic_frame: u64,
    last_frames: u64
}
impl CMDRendererProfiler{
    fn new() -> Self{
        Self{
            frame_profile_start: Instant::now(),
            last_check_frame: 0,
            last_logic_frame: 0,
            last_frames: 1
        }
    }
    fn start_frame_profile(&mut self){
        self.frame_profile_start = Instant::now();
    }
    fn stop_frame_profile(&self) -> std::time::Duration{
        self.frame_profile_start.elapsed()
    }
    fn update(&mut self, frame: u64, logic: u64){
        if !(self.last_logic_frame != logic && logic % 20 == 0) {
            return;
        }
        self.last_frames = frame - self.last_check_frame;
        self.last_check_frame = frame;
        self.last_logic_frame = logic;
    }
}