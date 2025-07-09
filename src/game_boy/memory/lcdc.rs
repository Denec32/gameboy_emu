pub struct Lcdc {
    lcd_ppu_enabled: bool,
    window_tile_map_are: bool,
    window_enable: bool,
    bg_window_tile_data_area: bool,
    bg_tile_map_are: bool,
    obj_size: bool,
    obj_enable: bool,
    bg_window_enable_priority: bool,
} 

impl Lcdc {
    pub fn default() -> Lcdc {
        Lcdc {
            lcd_ppu_enabled: false,
            window_tile_map_are: false,
            window_enable: false,
            bg_window_tile_data_area: false,
            bg_tile_map_are: false,
            obj_size: false,
            obj_enable: false,
            bg_window_enable_priority: false,
        }
    }
    
    pub fn set_flags(&mut self, flags: u8) {
        self.lcd_ppu_enabled = flags & (1 << 7) != 0;
        self.window_tile_map_are = flags & (1 << 6) != 0;
        self.window_enable = flags & (1 << 5) != 0;
        self.bg_window_tile_data_area = flags & (1 << 4) != 0;
        self.bg_tile_map_are = flags & (1 << 3) != 0;
        self.obj_size = flags & (1 << 2) != 0;
        self.obj_enable = flags & (1 << 1) != 0;
        self.bg_window_enable_priority = flags & (1 << 0) != 0;
    }
}
