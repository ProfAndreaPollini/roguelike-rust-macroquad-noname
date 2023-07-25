use macroquad::{
    prelude::{Color, Rect, Vec2, WHITE},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, Texture2D},
};
use zorder::coord_of;

use crate::engine::{core::camera::Camera, texture_manager::TextureManager};

use super::{tile::Tile, Map};

#[derive(Debug, Default)]
pub struct MapRenderer {}

impl MapRenderer {
    pub fn render(&self, map: &Map, camera: &Camera, texture_manager: &TextureManager) {
        let visible_area = camera.visible_area(texture_manager.cell_size);
        let viewport = camera.viewport;
        let zoom = camera.zoom;
        let texture = &texture_manager.texture;

        for (index, tile) in map.tiles_visible_from(visible_area) {
            let (x, y) = coord_of(index);

            let sprite_name: Option<&str> = tile.sprite_name();

            if sprite_name.is_none() {
                continue;
            }

            let sprite_rect = texture_manager.get_sprite(sprite_name.unwrap());

            let sprite_x = x as f32 * texture_manager.cell_size;
            let sprite_y = y as f32 * texture_manager.cell_size;

            let screen_x = (sprite_x) * zoom - camera.position.x / camera.zoom + viewport.x;
            let screen_y = (sprite_y) * zoom - camera.position.y / camera.zoom + viewport.y;

            let cell_size = texture_manager.cell_size * zoom;

            let screen_pos = camera.world_to_viewport(Vec2::new(sprite_x, sprite_y));

            self.render_tile(
                screen_pos.x,
                screen_pos.y,
                tile,
                texture,
                sprite_rect,
                cell_size,
            );

            if tile.explored() && !tile.visible() {
                self.highlight_tile(
                    Vec2::new(x as f32, y as f32),
                    texture_manager,
                    camera,
                    Color::new(0.0, 0.0, 0.0, 0.5),
                );
            }

            if !tile.explored() && !tile.visible() {
                self.highlight_tile(
                    Vec2::new(x as f32, y as f32),
                    texture_manager,
                    camera,
                    Color::new(1.0, 1.0, 0.0, 0.9),
                );
            }
        }
    }

    pub fn highlight_tile(
        &self,
        cell: Vec2,
        texture_manager: &TextureManager,
        camera: &Camera,
        color: Color,
    ) {
        let sprite_x = cell.x * texture_manager.cell_size;
        let sprite_y = cell.y * texture_manager.cell_size;

        let screen_x =
            (sprite_x) * camera.zoom - camera.position.x / camera.zoom + camera.viewport.x;
        let screen_y =
            (sprite_y) * camera.zoom - camera.position.y / camera.zoom + camera.viewport.y;

        let screen_pos = camera.world_to_viewport(Vec2::new(sprite_x, sprite_y));
        let cell_size = texture_manager.cell_size * camera.zoom;

        draw_rectangle(
            screen_pos.x,
            screen_pos.y,
            cell_size,
            cell_size,
            macroquad::color::Color::new(color.r, color.g, color.b, 0.5),
        );
    }

    fn render_tile(
        &self,
        x: f32,
        y: f32,
        tile: &Tile,
        texture: &Texture2D,
        sprite_rect: Rect,
        cell_size: f32,
    ) {
        draw_texture_ex(
            *texture,
            x,
            y,
            WHITE,
            macroquad::prelude::DrawTextureParams {
                source: Some(sprite_rect),
                dest_size: Some(Vec2::new(cell_size, cell_size)),
                ..Default::default()
            },
        );
    }

    fn render_tile_items(&self, x: f32, y: f32, tile: &Tile, texture: &Texture2D) {}
}

pub fn render_entity(
    cell: Vec2,
    sprite_rect: Rect,
    texture: &Texture2D,
    cell_size: f32,
    camera: &Camera,
) {
    let sprite_x = cell.x * cell_size;
    let sprite_y = cell.y * cell_size;

    let screen_x = (sprite_x) * camera.zoom - camera.position.x + camera.viewport.x;
    let screen_y = (sprite_y) * camera.zoom - camera.position.y + camera.viewport.y;

    let screen_pos = camera.world_to_viewport(Vec2::new(sprite_x, sprite_y));

    let cell_size = cell_size * camera.zoom;

    draw_texture_ex(
        *texture,
        screen_pos.x,
        screen_pos.y,
        WHITE,
        macroquad::prelude::DrawTextureParams {
            source: Some(sprite_rect),
            dest_size: Some(Vec2::new(cell_size, cell_size)),
            ..Default::default()
        },
    );
}
