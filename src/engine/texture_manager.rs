#![allow(dead_code, unused_variables)]

use macroquad::{
    miniquad::Texture,
    prelude::{Rect, Vec2},
    texture::{load_image, load_texture, Texture2D},
};

pub struct TextureManager {
    pub texture: Texture2D,
    pub cell_size: f32,
    pub scale: f32,
    pub tile_spacing: f32,
}

impl TextureManager {
    pub async fn new(path: &str, cell_size: f32, scale: f32, tile_spacing: f32) -> Self {
        let image = load_image(path).await.unwrap();

        let texture = Texture2D::from_image(&image);

        Self {
            texture,
            cell_size,
            scale,
            tile_spacing,
        }
    }

    pub fn tile_coords(&self, row: u32, col: u32) -> Rect {
        let x = col as f32 * (self.cell_size + self.tile_spacing) as f32;
        let y = row as f32 * (self.cell_size + self.tile_spacing) as f32;
        Rect::new(x, y, self.cell_size as f32, self.cell_size as f32)
    }

    pub fn cell_output_size(&self) -> Vec2 {
        Vec2 {
            x: self.cell_size * self.scale,
            y: self.cell_size * self.scale,
        }
    }
}
