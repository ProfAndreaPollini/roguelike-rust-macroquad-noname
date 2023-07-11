#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use macroquad::{
    prelude::{Color, BLUE, GREEN},
    shapes::draw_rectangle,
};
use noise::{NoiseFn, Perlin, Seedable};

use zorder::{coord_of, index_of};

#[derive(Debug)]
pub struct Tile {
    color: Color,
}

impl Tile {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            color: Color::new(0.0, 0.0, 0.0, 1.0),
        }
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

    pub fn generate(width: u32, height: u32) -> Self {
        let mut map = Self::new(width, height);

        let perlin = Perlin::new(1);
        let mut seed = 0;
        let mut noise = |x, y| {
            seed += 1;
            perlin.set_seed(seed);
            perlin.get([x as f64, y as f64])
        };

        for x in 0..width {
            for y in 0..height {
                let tile = if noise(x as f64 / 10., y as f64 / 10.) > 0. {
                    Tile::new(BLUE)
                } else {
                    Tile::new(GREEN)
                };

                map.add_tile(x as u16, y as u16, tile);
            }
        }

        map
    }

    pub fn draw(&self, texture_manager: &crate::engine::texture_manager::TextureManager) {
        for (index, tile) in &self.tiles {
            let (x, y) = coord_of(*index);
            draw_rectangle(
                x as f32 * texture_manager.cell_size * texture_manager.scale,
                y as f32 * texture_manager.cell_size * texture_manager.scale,
                texture_manager.cell_size * texture_manager.scale,
                texture_manager.cell_size * texture_manager.scale,
                tile.color,
            )
        }
    }

    pub fn add_tile(&mut self, x: u16, y: u16, tile: Tile) {
        self.tiles.insert(index_of((x, y)), tile);
    }

    pub fn tile_at(&self, x: u16, y: u16) -> Option<&Tile> {
        self.tiles.get(&index_of((x, y)))
    }
}
