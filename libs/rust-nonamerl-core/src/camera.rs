use crate::{Dimension2D, IntExtent2D, Map};

// Definizione della struttura del viewport
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn contains_screen_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

pub trait Camera {
    fn from_viewport(
        position_x: f32,
        position_y: f32,
        viewport: &Viewport,
        zoom_scale: f32,
    ) -> Self;
    fn world_to_viewport(&self, world_x: f32, world_y: f32, viewport: &Viewport) -> (f32, f32);
    fn viewport_to_world(
        &self,
        viewport_x: f32,
        viewport_y: f32,
        viewport: &Viewport,
    ) -> (f32, f32);
    fn center_on_world_point(&mut self, target_x: f32, target_y: f32, viewport: &Viewport);
    fn get_visibile_extent(&self, viewport: &Viewport, cell_w: usize, cell_h: usize)
        -> IntExtent2D;
}

// Implementazione della struttura della camera
pub struct Camera2D {
    pub position_x: f32,
    pub position_y: f32,
    pub fov_width: f32,
    pub fov_height: f32,
    pub zoom_scale: f32,
}

impl Camera for Camera2D {
    // Costruttore: inizializza il FOV basandosi sulle dimensioni del viewport
    fn from_viewport(
        position_x: f32,
        position_y: f32,
        viewport: &Viewport,
        zoom_scale: f32,
    ) -> Self {
        let aspect_ratio = viewport.width / viewport.height;
        let fov_width = viewport.width;
        let fov_height = viewport.width / aspect_ratio;

        Camera2D {
            position_x,
            position_y,
            fov_width,
            fov_height,
            zoom_scale,
        }
    }

    // Metodo per proiettare i punti del mondo nel viewport (come prima)
    fn world_to_viewport(&self, world_x: f32, world_y: f32, viewport: &Viewport) -> (f32, f32) {
        let camera_relative_x = world_x - self.position_x;
        let camera_relative_y = world_y - self.position_y;

        let normalized_x = camera_relative_x / (self.fov_width * self.zoom_scale);
        let normalized_y = camera_relative_y / (self.fov_height * self.zoom_scale);

        let viewport_x_pos = viewport.x + normalized_x * viewport.width;
        let viewport_y_pos = viewport.y + normalized_y * viewport.height;

        (viewport_x_pos, viewport_y_pos)
    }

    // Metodo per posizionare la camera in modo tale da avere il punto specifico al centro del FOV
    fn center_on_world_point(&mut self, target_x: f32, target_y: f32, viewport: &Viewport) {
        // Calcoliamo le coordinate relative della camera rispetto al punto specifico nel mondo
        let camera_relative_x = target_x - (self.fov_width * self.zoom_scale) / 2.0;
        let camera_relative_y = target_y - (self.fov_height * self.zoom_scale) / 2.0;

        // Impostiamo la posizione della camera in modo che il punto specifico sia al centro del FOV
        self.position_x = camera_relative_x;
        self.position_y = camera_relative_y;

        // Ora, assicuriamoci che la camera sia ancora all'interno del mondo di gioco
        // Eseguiamo questo calcolo tenendo conto delle dimensioni del viewport
        // let min_x = 0.0;
        // let min_y = 0.0;
        // let max_x = viewport.width - (self.fov_width * self.zoom_scale);
        // let max_y = viewport.height - (self.fov_height * self.zoom_scale);

        // self.position_x = self.position_x.max(min_x).min(max_x);
        // self.position_y = self.position_y.max(min_y).min(max_y);
    }

    // Metodo per convertire le coordinate dal viewport alle coordinate nel mondo di gioco
    fn viewport_to_world(
        &self,
        viewport_x: f32,
        viewport_y: f32,
        viewport: &Viewport,
    ) -> (f32, f32) {
        // Calcoliamo le coordinate normalizzate rispetto alle dimensioni del viewport
        let normalized_x = (viewport_x - viewport.x) / viewport.width;
        let normalized_y = (viewport_y - viewport.y) / viewport.height;

        // Calcoliamo le coordinate relative alla camera
        let camera_relative_x = normalized_x * self.fov_width * self.zoom_scale;
        let camera_relative_y = normalized_y * self.fov_height * self.zoom_scale;

        // Calcoliamo le coordinate assolute nel mondo di gioco
        let world_x = self.position_x + camera_relative_x;
        let world_y = self.position_y + camera_relative_y;

        (world_x, world_y)
    }

    fn get_visibile_extent(
        &self,
        viewport: &Viewport,
        cell_w: usize,
        cell_h: usize,
    ) -> IntExtent2D {
        let min_cell = self.viewport_to_world(viewport.x, viewport.y, viewport);
        let max_cell = self.viewport_to_world(
            viewport.x + viewport.width,
            viewport.y + viewport.height,
            viewport,
        );

        let min_cell = (
            (min_cell.0 / cell_w as f32) as i32,
            (min_cell.1 / cell_h as f32) as i32,
        );

        let max_cell = (
            (max_cell.0 / cell_w as f32) as i32,
            (max_cell.1 / cell_h as f32) as i32,
        );
        IntExtent2D::new(
            min_cell.0,
            min_cell.1,
            max_cell.0.abs_diff(min_cell.0) as usize,
            max_cell.1.abs_diff(min_cell.1) as usize,
        )
    }

    // Metodo per ricavare l'aspect ratio della camera dal viewport (come prima)
}

impl Camera2D {
    fn aspect_ratio(&self, viewport: &Viewport) -> f32 {
        viewport.width / viewport.height
    }
}
