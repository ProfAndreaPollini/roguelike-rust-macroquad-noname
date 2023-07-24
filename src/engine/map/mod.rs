#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

pub mod bresenham;
pub mod builder;
pub mod cell;
pub mod renderer;

pub mod tile;

use macroquad::{
    prelude::{Rect, RectOffset, WHITE},
    shapes::draw_rectangle,
    texture::draw_texture_ex,
};
use tile::Tile;

use zorder::{coord_of, index_of};

use self::{cell::Cell, tile::CellType};

use super::{core::entity::Entity, viewport::Viewport};

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

    pub fn tile_at_mut(&mut self, x: u16, y: u16) -> Option<&mut Tile> {
        self.tiles.get_mut(&index_of((x, y)))
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: MapTiles,
    pub cells: HashMap<u32, Cell>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: MapTiles::new(),
            cells: HashMap::new(),
        }
    }

    /// Returns a vector of tiles and their indices that are visible from a given rectangle.
    ///
    /// # Arguments
    ///
    /// * `rect` - A `Rect` struct representing the area to check for visible tiles.
    ///
    /// # Returns
    ///
    /// A vector of tuples, where the first element is the index of the tile and the second element is a reference to the tile itself.
    pub fn tiles_visible_from(&self, rect: Rect) -> Vec<(u32, &Tile)> {
        let mut tiles = vec![];
        let start_x = rect.x as u32;
        let start_y = rect.y as u32;
        let end_x = (rect.x + rect.w) as u32;
        let end_y = (rect.y + rect.h) as u32;

        // println!(
        //     "tiles_visible_from ->{} {} {} {}",
        //     start_x, start_y, end_x, end_y
        // );

        for x in start_x..end_x {
            for y in start_y..end_y {
                let tile = self.tile_at(x as u16, y as u16);
                if tile.is_none() {
                    // print!("no tile at {} {}", x, y);
                    continue;
                }
                // println!("tile at {} {} [{}]", x, y, tiles.len());
                tiles.push((index_of((x as u16, y as u16)), tile.unwrap()));
            }
        }

        tiles
    }

    pub fn set_tile_visible(&mut self, x: u16, y: u16, visible: bool) {
        let binding = self.tiles.tile_at_mut(x, y);
        // println!("{:?}", binding);
        // println!("{} {}", x, y);
        let tile = match binding {
            Some(t) => t,
            None => return,
        };

        tile.set_visible(visible);
        tile.set_explored(true);
        // println!("tile = {:?}", tile);
    }

    pub fn set_all_tiles_visibility(&mut self, visible: bool) {
        self.tiles.tiles.iter_mut().for_each(|(_, tile)| {
            tile.set_visible(visible);
            // tile.set_explored(true);
        });
    }

    pub fn set_tile_range_visibility(
        &mut self,
        start_x: u32,
        start_y: u32,
        fov_size: u32,
        visible: bool,
    ) {
        for x in start_x - fov_size + 1_u32..start_x + fov_size {
            for y in start_y - fov_size + 1_u32..start_y + fov_size {
                // print!("{} {}", x, y);
                self.set_tile_visible(x as u16, y as u16, visible);
            }
        }
        // println!("-----");

        // self.tiles.tiles.iter_mut().for_each(|(_, tile)| {
        //     tile.set_visible(visible);
        //     // tile.set_explored(true);
        // });
    }

    pub fn get_cell(&self, x: u16, y: u16) -> Option<&Cell> {
        self.cells.get(&index_of((x, y)))
    }

    pub fn draw(
        &self,
        texture_manager: &crate::engine::texture_manager::TextureManager,
        viewport: &Viewport,
    ) {
        let texture = &texture_manager.texture;

        let center = viewport.center();
        for (index, tile) in viewport.filter_tiles(self) {
            //&self.tiles.tiles {
            let (x, y) = coord_of(index);

            let sprite = if tile.visible() {
                let s = tile.visible_sprite_name();
                Some(texture_manager.get_sprite(s))
            } else if tile.explored() {
                let s = tile.explored_sprite_name();
                Some(texture_manager.get_sprite(s))
            } else {
                None
            };

            if sprite.is_none() {
                continue;
            }

            draw_texture_ex(
                texture_manager.texture,
                (x as f32 + center.x) * texture_manager.cell_output_size().x,
                (y as f32 + center.y) * texture_manager.cell_output_size().y,
                WHITE,
                macroquad::prelude::DrawTextureParams {
                    source: sprite,
                    dest_size: Some(texture_manager.cell_output_size()),
                    ..Default::default()
                },
            );

            if !tile.items().is_empty() {
                let item = tile.items().first().unwrap();
                let sprite = texture_manager.get_sprite(item.sprite_name());
                draw_texture_ex(
                    *texture,
                    (x as f32 + center.x) * texture_manager.cell_output_size().x,
                    (y as f32 + center.y) * texture_manager.cell_output_size().y,
                    WHITE,
                    macroquad::prelude::DrawTextureParams {
                        source: Some(sprite),
                        dest_size: Some(texture_manager.cell_output_size()),
                        ..Default::default()
                    },
                );
            }

            // explored but not visible overlay
            if tile.explored() && !tile.visible() {
                draw_rectangle(
                    (x as f32 + center.x) * texture_manager.cell_output_size().x,
                    (y as f32 + center.y) * texture_manager.cell_output_size().y,
                    texture_manager.cell_output_size().x,
                    texture_manager.cell_output_size().y,
                    macroquad::color::Color::new(0., 0., 0., 0.5),
                );
            }

            // let s = format!("({x},{y})");
            // draw_text(
            //     &s,
            //     (x as f32 + center.x) * texture_manager.cell_output_size().x,
            //     (y as f32 + center.y) * texture_manager.cell_output_size().y,
            //     12.,
            //     WHITE,
            // );

            // draw viewport rectangle border
        }
    }

    pub fn add_tile(&mut self, x: u16, y: u16, tile: Tile) {
        //self.tiles.insert(index_of((x, y)), tile);
        self.tiles.add_tile(x, y, tile);
        self.cells.insert(index_of((x, y)), Cell::new(x, y));
    }

    pub fn tile_at(&self, x: u16, y: u16) -> Option<&Tile> {
        //self.tiles.get(&index_of((x, y)))
        self.tiles.tile_at(x, y)
    }

    pub fn is_valid_position(&self, entity: &Entity, x: i32, y: i32) -> bool {
        // let (x, y) = coord_of(index_of((x as u16, y as u16)));
        // let (x, y) = (x as i32, y as i32);
        let (width, height) = (self.width as i32, self.height as i32);

        if x < 0 || x >= width || y < 0 || y >= height {
            return false;
        }

        if let Some(tile) = self.tile_at(x as u16, y as u16) {
            if let CellType::Wall = tile.cell_type {
                return false;
            }
        }

        true
    }

    pub fn is_position_blocked(&self, col: u16, row: u16) -> bool {
        if let Some(tile) = self.tile_at(col, row) {
            if let CellType::Wall = tile.cell_type {
                return true;
            }
        }

        false
    }
}
