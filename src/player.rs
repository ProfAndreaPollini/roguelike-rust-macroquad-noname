use std::collections::HashMap;

use macroquad::{
    prelude::{is_key_pressed, vec2, KeyCode, Rect, Vec2, WHITE},
    texture::{draw_texture, draw_texture_ex},
};

use crate::{actions::ActionHandler, engine::texture_manager::TextureManager};

pub struct Player<'a> {
    sprites: HashMap<String, Rect>,
    texture_manager: &'a TextureManager,
    pub x: i32,
    pub y: i32,
}
impl<'a> Player<'a> {
    pub(crate) fn new(texture_manager: &'a TextureManager) -> Self {
        let mut sprites = HashMap::new();
        sprites.insert("idle".to_string(), texture_manager.tile_coords(17, 0));

        Self {
            sprites,
            texture_manager,
            x: 10,
            y: 10,
        }
    }

    pub fn add_sprite(&mut self, name: &str, row: u32, col: u32) {
        self.sprites
            .insert(name.to_string(), self.texture_manager.tile_coords(row, col));
    }

    pub fn handle_input(&mut self, action_handler: &mut ActionHandler) {
        if is_key_pressed(KeyCode::Right) {
            // self.x += 1;
            action_handler.add_action(crate::actions::Action::Move(1, 0));
        }
        if is_key_pressed(KeyCode::Left) {
            // self.x -= 1;
            action_handler.add_action(crate::actions::Action::Move(-1, 0));
        }
        if is_key_pressed(KeyCode::Up) {
            // self.y -= 1;
            action_handler.add_action(crate::actions::Action::Move(0, -1));
        }
        if is_key_pressed(KeyCode::Down) {
            // self.y += 1;
            action_handler.add_action(crate::actions::Action::Move(0, 1));
        }
    }

    pub fn draw(&self) {
        let texture = self.texture_manager.texture;
        draw_texture_ex(
            texture,
            self.x as f32 * self.texture_manager.cell_output_size().x,
            self.y as f32 * self.texture_manager.cell_output_size().y,
            WHITE,
            macroquad::prelude::DrawTextureParams {
                source: Some(self.sprites.get("idle").unwrap().clone()),
                dest_size: Some(self.texture_manager.cell_output_size()),
                ..Default::default()
            },
        );
    }
}
