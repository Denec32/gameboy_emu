enum Priority {
    No, 
    DrawOver,
}

enum Flip {
    Normal,
    Mirror,
}

enum DmgPalette {
    OBP0,
    OBP1,
}

pub struct ObjectAttributes {
    y_position: u8,
    x_position: u8,
    tile_index: u8,
    
    priority: Priority,
    y_flip: Flip,
    x_flip: Flip,
    dmg_palette: DmgPalette,
}

impl ObjectAttributes {
    pub fn new() -> ObjectAttributes {
        ObjectAttributes{
            y_position: 0,
            x_position: 0,
            tile_index: 0,
            priority: Priority::No,
            y_flip: Flip::Normal,
            x_flip: Flip::Normal,
            dmg_palette: DmgPalette::OBP0,
        }
    }
}