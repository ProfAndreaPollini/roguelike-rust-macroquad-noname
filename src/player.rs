use macroquad::{
    prelude::{is_key_pressed, KeyCode, WHITE},
    texture::draw_texture_ex,
};

use crate::{
    actions::{ActionHandler, Move},
    engine::{core::Entity, texture_manager::TextureManager},
};

#[derive(Debug, Clone)]
pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Entity for Player {
    fn x(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn y(&mut self) -> &mut i32 {
        &mut self.y
    }

    fn as_player_mut(&mut self) -> Option<&mut Player> {
        Some(self)
    }
}

impl Player {
    pub fn new() -> Self {
        // let sprites = HashMap::new();
        // sprites.insert("idle".to_string(), texture_manager.tile_coords(17, 0));

        Self {
            // sprites,
            // texture_manager,
            x: 12,
            y: 12,
        }
    }

    pub fn handle_input(&mut self, action_handler: &mut ActionHandler) {
        if is_key_pressed(KeyCode::Right) {
            // self.x += 1;
            action_handler.add_action(crate::actions::Action::Move(Move { dx: 1, dy: 0 }));
        }
        if is_key_pressed(KeyCode::Left) {
            // self.x -= 1;
            action_handler.add_action(crate::actions::Action::Move(Move { dx: -1, dy: 0 }));
        }
        if is_key_pressed(KeyCode::Up) {
            // self.y -= 1;
            action_handler.add_action(crate::actions::Action::Move(Move { dx: 0, dy: -1 }));
        }
        if is_key_pressed(KeyCode::Down) {
            // self.y += 1;
            action_handler.add_action(crate::actions::Action::Move(Move { dx: 0, dy: 1 }));
        }
    }

    pub fn draw(&self, texture_manager: &TextureManager) {
        let texture = texture_manager.texture;
        let idle_sprite = texture_manager.get_sprite("idle");

        // println!("idle_sprite: {:?}", idle_sprite);

        draw_texture_ex(
            texture,
            self.x as f32 * texture_manager.cell_output_size().x,
            self.y as f32 * texture_manager.cell_output_size().y,
            WHITE,
            macroquad::prelude::DrawTextureParams {
                source: Some(idle_sprite),
                dest_size: Some(texture_manager.cell_output_size()),
                ..Default::default()
            },
        );
    }
}
