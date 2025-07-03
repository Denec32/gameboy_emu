pub struct PPU {
    
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

impl PPU {
    pub fn new() -> PPU {
        PPU{}
    }
    
    pub fn draw_frame(&self) {
        for i in 1..=144 {
            self.mode2();
            self.mode3();
            self.mode0();
        }
        self.mode1();
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
    
    /// Wait until the end of scanline
    /// Duration is 376 - mode 3's duration
    /// Can access VRAM, OAM and CGB palettes
    fn mode0(&self) {

    }
    
    /// Wait until the next frame
    /// Duration is 456 * 10 scanlines
    /// Can access VRAM, OAM and CGB palettes 
    fn mode1(&self) {

    }
}