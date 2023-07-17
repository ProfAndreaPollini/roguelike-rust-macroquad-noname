use macroquad::{prelude::WHITE, texture::draw_texture_ex};

use crate::engine::{
    core::entity::{draw_sprite, Drawable, Updatable},
    texture_manager::TextureManager,
    viewport::Viewport,
};

#[derive(Debug, Clone, Default)]
pub struct NPC {
    pub x: i32,
    pub y: i32,
}

impl Drawable for NPC {
    fn draw(&self, texture_manager: &TextureManager, viewport: &Viewport) {
        let texture = &texture_manager.texture;
        let idle_sprite = texture_manager.get_sprite("npc01");

        let center = viewport.center();

        // draw_texture_ex(
        //     texture,
        //     (self.x as f32 + center.x) * texture_manager.cell_output_size().x,
        //     (self.y as f32 + center.y) * texture_manager.cell_output_size().y,
        //     WHITE,
        //     macroquad::prelude::DrawTextureParams {
        //         source: Some(idle_sprite),
        //         dest_size: Some(texture_manager.cell_output_size()),
        //         ..Default::default()
        //     },
        // );

        draw_sprite(
            texture,
            self.x,
            self.y,
            viewport,
            texture_manager.cell_output_size(),
            idle_sprite,
        );
    }
}

impl Updatable for NPC {
    fn update(&mut self) {}
    fn next_action(&self) -> Option<crate::actions::Action> {
        None
    }

    fn position(&self) -> Option<(i32, i32)> {
        Some((self.x, self.y))
    }
}
