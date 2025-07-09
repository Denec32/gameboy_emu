pub struct PPU {
    acc: i32,
    current_scanline: u8
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

impl PPU {
    pub fn new() -> PPU {
        PPU{ acc: 0, current_scanline: 0 }
    }

    pub fn step(&mut self, cycles: i32) {
        self.acc += 4 * cycles;
        if self.acc >= 456 {
            self.acc -= 456;
            self.draw_line();
        }
    }

    fn draw_line(&mut self) {
        if self.current_scanline <= 143 {
            self.mode2();
            self.mode3();
        }
        self.current_scanline += 1;

        if self.current_scanline == 154 {
            self.draw_frame();
            self.current_scanline = 0;
        }
    }

    pub fn draw_frame(&self) {
        println!("drawing frame");
    }
    
    /// Search OBJs which overlap current line.
    /// Duration is 80 dots.
    /// Can access VRAM and CGB palettes
    fn mode2(&self) {

    }
    
    /// Send pixels to LCD
    /// Duration is between 172 and 289 dots
    /// can't access video memory
    fn mode3(&self) {

    }
}