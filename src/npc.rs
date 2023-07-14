use macroquad::{prelude::WHITE, texture::draw_texture_ex};

use crate::{
    engine::{core::Entity, texture_manager::TextureManager},
    player::Player,
};

#[derive(Debug, Clone)]
pub struct NPC {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

impl NPC {
    pub fn new(x: i32, y: i32, name: String) -> Self {
        Self { x, y, name }
    }

    pub fn draw(&self, texture_manager: &TextureManager) {
        let texture = texture_manager.texture;
        let idle_sprite = texture_manager.get_sprite("npc01");

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

impl Entity for NPC {
    fn x(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn y(&mut self) -> &mut i32 {
        &mut self.y
    }

    fn as_player_mut(&mut self) -> Option<&mut Player> {
        None
    }
}
