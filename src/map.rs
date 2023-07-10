use std::collections::HashMap;

use zorder::index_of;

#[derive(Debug)]
pub struct Tile {}

impl Tile {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: HashMap<u32, Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, x: u16, y: u16, tile: Tile) {
        self.tiles.insert(index_of((x, y)), tile);
    }

    pub fn tile_at(&self, x: u16, y: u16) -> Option<&Tile> {
        self.tiles.get(&index_of((x, y)))
    }
}
