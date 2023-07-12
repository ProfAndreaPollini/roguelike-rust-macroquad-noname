#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

use macroquad::{
    prelude::{Rect, Vec2},
    texture::{load_image, Texture2D},
};

/// A struct that manages a texture atlas and provides methods for accessing individual sprites.
/// A struct that manages a texture atlas and provides methods for accessing individual sprites.
pub struct TextureManager {
    /// The texture atlas managed by the texture manager.
    pub texture: Texture2D,
    /// The size of each cell in the texture atlas.
    pub cell_size: f32,
    /// The scale factor to apply to each cell when rendering.
    pub scale: f32,
    /// The amount of spacing to add between each tile when rendering.
    pub tile_spacing: f32,
    /// A hashmap that stores the coordinates of each sprite in the texture atlas.
    sprites: HashMap<String, Rect>,
}

impl TextureManager {
    pub async fn new(path: &str, cell_size: f32, scale: f32, tile_spacing: f32) -> Self {
        let image = load_image(path).await.unwrap();

        let texture = Texture2D::from_image(&image);

        let sprites = HashMap::new();

        Self {
            texture,
            cell_size,
            scale,
            tile_spacing,
            sprites,
        }
    }

    pub fn add_sprite(&mut self, name: &str, row: u32, col: u32) {
        self.sprites
            .insert(name.to_string(), self.tile_coords(row, col));
    }

    pub fn get_sprite(&self, name: &str) -> Rect {
        self.sprites[name]
    }

    /// Returns the coordinates of a tile in the texture atlas based on its row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - A u32 that represents the row of the tile in the texture atlas.
    /// * `col` - A u32 that represents the column of the tile in the texture atlas.
    ///
    /// # Returns
    ///
    /// A `Rect` struct that represents the coordinates of the tile in the texture atlas.
    pub fn tile_coords(&self, row: u32, col: u32) -> Rect {
        let x = col as f32 * (self.cell_size + self.tile_spacing);
        let y = row as f32 * (self.cell_size + self.tile_spacing);
        Rect::new(x, y, self.cell_size, self.cell_size)
    }

    /// Loads sprite information from a JSON file and adds them to the texture manager.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the JSON file.
    pub fn load_from_json(&mut self, path: &str) {
        let json = std::fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&json).unwrap();

        let sprites = json["sprites"].as_object().unwrap();

        for (name, sprite) in sprites {
            let row = sprite["row"].as_u64().unwrap() as u32;
            let col = sprite["col"].as_u64().unwrap() as u32;
            self.add_sprite(name, row, col);
        }
        println!("{:?}", self.sprites)
    }

    pub fn cell_output_size(&self) -> Vec2 {
        Vec2 {
            x: self.cell_size * self.scale,
            y: self.cell_size * self.scale,
        }
    }
}
