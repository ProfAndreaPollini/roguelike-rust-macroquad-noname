#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

pub mod builder;
pub mod tile;

use tile::Tile;

use macroquad::{prelude::WHITE, texture::draw_texture_ex};
use noise::{NoiseFn, Perlin, Seedable};

use zorder::{coord_of, index_of};

use crate::engine::core::Entity;

use self::tile::{CellType, Visibility};

#[derive(Debug, Clone)]
pub struct MapTiles {
    pub tiles: HashMap<u32, Tile>,
}

impl MapTiles {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, x: u16, y: u16, tile: Tile) {
        let idx = index_of((x, y));
        if !self.tiles.contains_key(&idx) {
            self.tiles.insert(idx, tile);
        } else {
            //remove old tile
            self.tiles.remove(&idx);
            //add new tile
            self.tiles.insert(idx, tile);
        }
        // self.tiles.insert(index_of((x, y)), tile);
    }

    pub fn tile_at(&self, x: u16, y: u16) -> Option<&Tile> {
        self.tiles.get(&index_of((x, y)))
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: MapTiles,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: MapTiles::new(),
        }
    }

    pub fn generate(width: u32, height: u32) -> Self {
        let mut map = Self::new(width, height);

        let perlin = Perlin::new(1);
        let mut seed = 0;
        let mut noise = |x, y| {
            seed += 1;
            perlin.set_seed(seed);
            perlin.get([x, y])
        };

        for x in 0..width {
            for y in 0..height {
                let tile = if noise(x as f64 / 2., y as f64 / 2.).abs() > 0. {
                    Tile::new("floor".to_string())
                    // tile.add_sprite("floor", 2, 2);
                } else {
                    let mut t = Tile::new("wall".to_string());
                    t.cell_type = CellType::Wall;
                    t
                };

                map.add_tile(x as u16, y as u16, tile);
            }
        }

        map
    }

    pub fn draw(&self, texture_manager: &crate::engine::texture_manager::TextureManager) {
        let texture = texture_manager.texture;
        for (index, tile) in &self.tiles.tiles {
            let (x, y) = coord_of(*index);
            //texture_manager.get_sprite(&tile.visibility);
            let sprite = match &tile.visibility {
                Visibility::Hidden(sprite_name) => texture_manager.get_sprite(sprite_name),
                Visibility::Visible(sprite_name) => texture_manager.get_sprite(sprite_name),
            };
            // draw_rectangle(
            //     x as f32 * texture_manager.cell_size * texture_manager.scale,
            //     y as f32 * texture_manager.cell_size * texture_manager.scale,
            //     texture_manager.cell_size * texture_manager.scale,
            //     texture_manager.cell_size * texture_manager.scale,
            //     tile.color,
            // )
            draw_texture_ex(
                texture,
                x as f32 * texture_manager.cell_output_size().x,
                y as f32 * texture_manager.cell_output_size().y,
                WHITE,
                macroquad::prelude::DrawTextureParams {
                    source: Some(sprite),
                    dest_size: Some(texture_manager.cell_output_size()),
                    ..Default::default()
                },
            );
        }
    }

    pub fn add_tile(&mut self, x: u16, y: u16, tile: Tile) {
        //self.tiles.insert(index_of((x, y)), tile);
        self.tiles.add_tile(x, y, tile);
    }

    pub fn tile_at(&self, x: u16, y: u16) -> Option<&Tile> {
        //self.tiles.get(&index_of((x, y)))
        self.tiles.tile_at(x, y)
    }

    pub fn is_valid_position(&self, entity: &dyn Entity, x: i32, y: i32) -> bool {
        // let (x, y) = coord_of(index_of((x as u16, y as u16)));
        // let (x, y) = (x as i32, y as i32);
        let (width, height) = (self.width as i32, self.height as i32);

        if x < 0 || x >= width || y < 0 || y >= height {
            return false;
        }

        if let Some(tile) = self.tile_at(x as u16, y as u16) {
            if let Visibility::Hidden(_) = tile.visibility {
                return true;
            }
            if let CellType::Wall = tile.cell_type {
                return false;
            }
        }

        true
    }
}
