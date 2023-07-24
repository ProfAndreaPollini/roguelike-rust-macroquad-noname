use crate::engine::{
    core::{
        camera::Camera,
        entity::{draw_sprite, Drawable, Updatable},
    },
    texture_manager::TextureManager,
    viewport::Viewport,
};

#[derive(Debug, Clone, Default)]
pub struct NPC {
    pub x: i32,
    pub y: i32,
}

impl Drawable for NPC {
    fn draw(&self, texture_manager: &TextureManager, camera: &Camera) {
        let texture = &texture_manager.texture;
        let idle_sprite = texture_manager.get_sprite("npc01");

        // let center = viewport.center();

        // draw_sprite(
        //     texture,
        //     self.x,
        //     self.y,
        //     viewport,
        //     texture_manager.cell_output_size(),
        //     idle_sprite,
        // );
    }
}

impl Updatable for NPC {
    fn position(&self) -> Option<(i32, i32)> {
        Some((self.x, self.y))
    }
}
