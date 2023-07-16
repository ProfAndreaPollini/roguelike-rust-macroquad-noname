use macroquad::{prelude::WHITE, texture::draw_texture_ex};

use crate::engine::{core::Entity, texture_manager::TextureManager, viewport::Viewport};

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

    fn draw(&self, texture_manager: &TextureManager, viewport: &Viewport) {
        let texture = texture_manager.texture;
        let idle_sprite = texture_manager.get_sprite("npc01");

        let center = viewport.center();

        draw_texture_ex(
            texture,
            (self.x as f32 + center.x) * texture_manager.cell_output_size().x,
            (self.y as f32 + center.y) * texture_manager.cell_output_size().y,
            WHITE,
            macroquad::prelude::DrawTextureParams {
                source: Some(idle_sprite),
                dest_size: Some(texture_manager.cell_output_size()),
                ..Default::default()
            },
        );
    }
}

// impl Entity for NPC {
//     fn x(&mut self) -> &mut i32 {
//         &mut self.x
//     }

//     fn y(&mut self) -> &mut i32 {
//         &mut self.y
//     }

//     fn as_player_mut(&mut self) -> Option<&mut Player> {
//         None
//     }
// }

impl Entity for NPC {
    fn draw(&self, texture_manager: &TextureManager, viewport: &crate::engine::viewport::Viewport) {
    }

    fn is_player(&self) -> bool {
        false
    }

    fn next_action(&self) -> Option<crate::actions::Action> {
        None
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
