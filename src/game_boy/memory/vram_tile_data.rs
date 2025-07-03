use crate::game_boy::memory::vram_tile_data::tile::Tile;

mod tile;

pub struct VramTileData {
    tiles: Vec<Tile>
}

impl VramTileData {
    pub fn new() -> VramTileData {
        let mut tiles: Vec<Tile> = Vec::new();
        for _ in 0..128 {
            tiles.push(Tile::new());
        }
        VramTileData { tiles }
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        todo!()
    }
}

