use macroquad::prelude::{Color, Vec2, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text_ex;
use macroquad::texture::{draw_texture_ex, DrawTextureParams};

use crate::dimension::Dimension2;
use crate::{Camera, Camera2D, Dimension2D, Map, SpriteSheet, Tile, TileSpriteInfo, Viewport};
#[derive(Debug, Copy, Clone)]
pub enum RenderOp<T: Tile> {
    DrawTile(i32, i32, T),
    DrawRectangle,
    DrawCircle,
    HighlightCell(i32, i32),
    FillCell(i32, i32, Color),
}

#[derive(Debug, Copy, Clone)]
pub struct Renderer {
    pub cell_size: Dimension2D<usize>,
}

impl Renderer {
    pub fn from_map_cell_size(cell_size: Dimension2D<usize>) -> Self {
        Self { cell_size }
    }

    pub fn from_map<T: Tile>(map: &Map<T>) -> Self {
        Self {
            cell_size: map.cell_size(),
        }
    }

    pub fn batch_render<T: Tile>(
        &self,
        camera: &Camera2D,
        viewport: &Viewport,
        sprites: &SpriteSheet,
        render_ops: &Vec<RenderOp<T>>,
    ) {
        for render_op in render_ops {
            match render_op {
                RenderOp::FillCell(x, y, color) => {
                    let (viewport_x, viewport_y) = camera.world_to_viewport(
                        *x as f32 * self.cell_size.width() as f32,
                        *y as f32 * self.cell_size.height() as f32,
                        viewport,
                    );

                    draw_rectangle(
                        viewport_x,
                        viewport_y,
                        self.cell_size.width() as f32 / camera.zoom_scale,
                        self.cell_size.height() as f32 / camera.zoom_scale,
                        *color,
                    );
                }
                RenderOp::DrawTile(x, y, tile) => {
                    let (viewport_x, viewport_y) = camera.world_to_viewport(
                        *x as f32 * self.cell_size.width() as f32,
                        *y as f32 * self.cell_size.height() as f32,
                        viewport,
                    );

                    let sprite_info = tile.sprite_info();

                    match sprite_info {
                        TileSpriteInfo::None => {}
                        TileSpriteInfo::Fill(color) => {
                            draw_rectangle(
                                viewport_x,
                                viewport_y,
                                self.cell_size.width() as f32 / camera.zoom_scale,
                                self.cell_size.height() as f32 / camera.zoom_scale,
                                color,
                            );
                        }
                        TileSpriteInfo::SpriteSheet(name) => {
                            let (rect, texture) = sprites.get_sprite(name);

                            draw_texture_ex(
                                texture,
                                viewport_x,
                                viewport_y,
                                WHITE,
                                DrawTextureParams {
                                    source: Some(*rect),
                                    dest_size: Some(Vec2::new(
                                        self.cell_size.width() as f32 / camera.zoom_scale,
                                        self.cell_size.height() as f32 / camera.zoom_scale,
                                    )),
                                    ..Default::default()
                                },
                            );
                        }
                        TileSpriteInfo::SingleSprite(texture) => {
                            draw_texture_ex(
                                &texture,
                                viewport_x,
                                viewport_y,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(
                                        self.cell_size.width() as f32 / camera.zoom_scale,
                                        self.cell_size.height() as f32 / camera.zoom_scale,
                                    )),
                                    ..Default::default()
                                },
                            );
                        }
                    };
                    // if !tile.is_visited() {
                    //     draw_rectangle(
                    //         viewport_x,
                    //         viewport_y,
                    //         self.cell_size.width() as f32 / camera.zoom_scale,
                    //         self.cell_size.height() as f32 / camera.zoom_scale,
                    //         Color {
                    //             r: 0.0,
                    //             g: 0.0,
                    //             b: 0.0,
                    //             a: 0.8,
                    //         },
                    //     );
                    // }

                    match (tile.is_visible(), tile.is_visited()) {
                        (false, true) => {
                            draw_rectangle(
                                viewport_x,
                                viewport_y,
                                self.cell_size.width() as f32 / camera.zoom_scale,
                                self.cell_size.height() as f32 / camera.zoom_scale,
                                Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.3,
                                },
                            );
                        }
                        (false, false) => {
                            draw_rectangle(
                                viewport_x,
                                viewport_y,
                                self.cell_size.width() as f32 / camera.zoom_scale,
                                self.cell_size.height() as f32 / camera.zoom_scale,
                                Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.8,
                                },
                            );
                        }
                        (_, _) => {}
                    }

                    // draw_rectangle(
                    //     viewport_x,
                    //     viewport_y,
                    //     self.cell_size.width() as f32 * camera.zoom_scale,
                    //     self.cell_size.height() as f32 * camera.zoom_scale,
                    //     Color {
                    //         r: 1.0,
                    //         g: 0.,
                    //         b: 0.,
                    //         a: 0.6,
                    //     },
                    // );
                }
                RenderOp::DrawRectangle => {}
                RenderOp::DrawCircle => {}
                RenderOp::HighlightCell(x, y) => {
                    let (viewport_x, viewport_y) = camera.world_to_viewport(
                        *x as f32 * self.cell_size.width() as f32,
                        *y as f32 * self.cell_size.height() as f32,
                        viewport,
                    );

                    draw_rectangle(
                        viewport_x,
                        viewport_y,
                        self.cell_size.width() as f32 / camera.zoom_scale,
                        self.cell_size.height() as f32 / camera.zoom_scale,
                        Color {
                            r: 1.0,
                            g: 0.,
                            b: 0.,
                            a: 0.6,
                        },
                    );
                }
            }
        }
    }
}

#[cfg(test)]

mod tests {

    use crate::{FovOccluder, IntExtent2D, Map, Visible, Visited};

    use super::*;

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct TestTile {}
    impl Tile for TestTile {}
    impl Visible for TestTile {}
    impl Visited for TestTile {}
    impl FovOccluder for TestTile {}

    #[test]
    fn test_renderer() {
        let map = Map::<TestTile>::new(IntExtent2D::new(0, 0, 10, 10), Dimension2D::new(24, 24));

        let renderer = Renderer::from_map_cell_size(map.cell_size());
        assert_eq!(renderer.cell_size, Dimension2D::new(24, 24));
    }

    #[test]
    fn test_renderer_from_map() {
        let map = Map::<TestTile>::new(IntExtent2D::new(0, 0, 10, 10), Dimension2D::new(24, 24));

        let renderer = Renderer::from_map(&map);
        assert_eq!(renderer.cell_size, Dimension2D::new(24, 24));
    }
}
