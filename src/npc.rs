use macroquad::prelude::Vec2;

use crate::engine::{
    core::{
        camera::Camera,
        direction::Direction,
        entity::{Drawable, EnergyBased, EntityTrait, Updatable},
    },
    map::renderer::render_entity,
    texture_manager::TextureManager,
};

#[derive(Debug, Clone, Default)]
pub struct NPC {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub energy: u32,
}

impl Drawable for NPC {
    fn draw(&self, texture_manager: &TextureManager, camera: &Camera) {
        let texture = &texture_manager.texture;
        let sprite_rect = texture_manager.get_sprite("idle");
        let cell_size = texture_manager.cell_size;

        // let sprite_x = self.x as f32 * cell_size;
        // let sprite_y = self.y as f32 * cell_size;
        // let screen_pos = camera.world_to_viewport(Vec2::new(sprite_x, sprite_y));

        render_entity(
            Vec2::new(self.x as f32, self.y as f32),
            sprite_rect,
            texture,
            cell_size,
            camera,
        );
    }
}

impl Updatable for NPC {
    fn position(&self) -> Option<(i32, i32)> {
        Some((self.x, self.y))
    }
}

impl EnergyBased for NPC {
    fn energy(&self) -> u32 {
        1
    }
    fn increase_energy(&mut self) {
        self.energy += 1;
    }
}

impl EntityTrait for NPC {}
