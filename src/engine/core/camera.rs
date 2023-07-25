use macroquad::prelude::{Rect, Vec2};

#[derive(Debug)]
pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
    pub viewport: Rect,
}

impl Camera {
    pub fn new(position: Vec2, zoom: f32, viewport: Rect) -> Self {
        Self {
            position,
            zoom,
            viewport,
        }
    }

    fn visible_area_size(&self, tile_size: f32) -> Vec2 {
        Vec2::new(
            self.viewport.w / (tile_size * self.zoom),
            self.viewport.h / (tile_size * self.zoom),
        )
    }

    pub fn visible_area(&self, tile_size: f32) -> Rect {
        let size = self.visible_area_size(tile_size);
        // println!("visible_area_size = {:?}", size);
        Rect::new(
            self.position.x / (tile_size * self.zoom),
            self.position.y / (tile_size * self.zoom),
            size.x,
            size.y,
        )
    }

    pub fn world_to_viewport(&self, pos: Vec2) -> Vec2 {
        let screen_x = (pos.x - self.position.x) * self.zoom + self.viewport.x;
        let screen_y = (pos.y - self.position.y) * self.zoom + self.viewport.y;
        Vec2::new(screen_x, screen_y)
    }

    pub fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        let world_x = (pos.x - self.viewport.x) / self.zoom + self.position.x;
        let world_y = (pos.y - self.viewport.y) / self.zoom + self.position.y;
        Vec2::new(world_x, world_y)
    }

    pub fn viewport_center(&self) -> Vec2 {
        Vec2::new(
            self.viewport.w / (2. * self.zoom),
            self.viewport.h / (2. * self.zoom),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::new(-150., -200.),
            zoom: 0.4,
            viewport: Rect::new(100.0, 100.0, 300.0, 400.0),
        }
    }
}
