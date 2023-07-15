use macroquad::prelude::{Rect, Vec2};
use zorder::{coord_of, index_of};

use super::map::{tile::Tile, Map};

#[derive(Debug, Clone)]
pub struct Viewport(Rect);

impl Viewport {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self(Rect::new(x, y, w, h))
    }

    pub fn set(&mut self, rect: Rect) {
        self.0 = rect;
    }

    pub fn get(&self) -> Rect {
        self.0
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.0.x = x - self.0.w / 2.0;
        self.0.y = y - self.0.h / 2.0;
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.0.x += dx;
        self.0.y += dy;
    }

    pub fn filter_tiles(&self, map: &Map) -> Vec<(u32, Tile)> {
        let mut tiles: Vec<(u32, Tile)> = vec![];

        // map.tiles.tiles.iter().for_each(|item| {
        //     let tile = item.1;
        //     let coords = coord_of(*item.0);
        //     let pos = Vec2::new(coords.0 as f32, coords.1 as f32);

        //     if self.0.contains(pos) {
        //         tiles.push(tile.clone());
        //     }
        // });

        let center = self.0.center();

        for x in self.0.left() as i32..self.0.right() as i32 {
            for y in self.0.top() as i32..self.0.bottom() as i32 {
                let pos = Vec2::new(x as f32, y as f32);
                let idx = index_of((x as u16, y as u16));

                if let Some(tile) = map.tile_at(x as u16, y as u16) {
                    tiles.push((idx, tile.clone()));
                }
            }
        }

        tiles
    }
}
