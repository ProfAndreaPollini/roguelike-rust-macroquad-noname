#![allow(dead_code)]
use macroquad::prelude::{Rect, Vec2};
use zorder::index_of;

use super::map::{tile::Tile, Map};

#[derive(Debug, Clone, Default)]
pub struct Viewport(Rect, Vec2);

impl Viewport {
    pub fn new(x: f32, y: f32, w: f32, h: f32, offset: Vec2) -> Self {
        Self(Rect::new(x, y, w, h), offset)
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

        // let center = self.0.center();

        for x in self.0.left() as i32..self.0.right() as i32 {
            for y in self.0.top() as i32..self.0.bottom() as i32 {
                // let pos = Vec2::new(x as f32, y as f32);
                let idx = index_of((x as u16, y as u16));

                if let Some(tile) = map.tile_at(x as u16, y as u16) {
                    tiles.push((idx, tile.clone()));
                }
            }
        }

        tiles
    }

    pub fn offset(&self) -> &Vec2 {
        &self.1
    }

    pub fn offset_mut(&mut self) -> &mut Vec2 {
        &mut self.1
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.0
    }

    pub fn center(&self) -> Vec2 {
        let offset = self.1;
        self.0.center() + offset
    }
}
