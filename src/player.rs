use macroquad::{
    prelude::{is_key_pressed, KeyCode, WHITE},
    texture::draw_texture_ex,
};

use crate::{
    actions::{ActionHandler, Move},
    engine::{
        core::entity::{draw_sprite, Drawable, Updatable},
        texture_manager::TextureManager,
        viewport::Viewport,
    },
};

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Drawable for Player {
    fn draw(&self, texture_manager: &TextureManager, viewport: &Viewport) {
        println!("Player draw");
        let texture = &texture_manager.texture;
        //...draw_sprite("idle", 0, 0);
        let idle_sprite = texture_manager.get_sprite("idle");

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
impl Updatable for Player {
    fn update(&mut self) {}

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn next_action(&self) -> Option<crate::actions::Action> {
        let mut action = None;
        if is_key_pressed(KeyCode::Right) {
            // self.x += 1;
            action = Some(crate::actions::Action::Move(Move { dx: 1, dy: 0 }));
        }
        if is_key_pressed(KeyCode::Left) {
            // self.x -= 1;
            //action_handler.add_action(crate::actions::Action::Move(Move { dx: -1, dy: 0 }));
            action = Some(crate::actions::Action::Move(Move { dx: -1, dy: 0 }));
        }
        if is_key_pressed(KeyCode::Up) {
            // self.y -= 1;
            //action_handler.add_action(crate::actions::Action::Move(Move { dx: 0, dy: -1 }));
            action = Some(crate::actions::Action::Move(Move { dx: 0, dy: -1 }));
        }
        if is_key_pressed(KeyCode::Down) {
            // self.y += 1;
            //action_handler.add_action(crate::actions::Action::Move(Move { dx: 0, dy: 1 }));
            action = Some(crate::actions::Action::Move(Move { dx: 0, dy: 1 }));
        }

        println!("Player next action: {:?}", action);

        action
    }

    fn position(&self) -> Option<(i32, i32)> {
        Some((self.x, self.y))
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

    // pub fn draw(&self, texture_manager: &TextureManager, viewport: &Viewport) {
    //     let texture = texture_manager.texture;
    //     //...draw_sprite("idle", 0, 0);
    //     let idle_sprite = texture_manager.get_sprite("idle");

    //     // println!("idle_sprite: {:?}", idle_sprite);
    //     // let center = -1.0 * viewport.get().center();
    //     // let offset = *viewport.offset();
    //     // let center = center + offset;
    //     let center = viewport.center();

    //     draw_texture_ex(
    //         texture,
    //         (self.x as f32 + center.x) * texture_manager.cell_output_size().x,
    //         (self.y as f32 + center.y) * texture_manager.cell_output_size().y,
    //         WHITE,
    //         macroquad::prelude::DrawTextureParams {
    //             source: Some(idle_sprite),
    //             dest_size: Some(texture_manager.cell_output_size()),
    //             ..Default::default()
    //         },
    //     );

    //     // draw_line(
    //     //     (viewport.offset().x + center.x) * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y + center.y) * texture_manager.cell_output_size().y,
    //     //     (viewport.offset().x + viewport.get().w + center.x)
    //     //         * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y + center.y) * texture_manager.cell_output_size().y,
    //     //     10.,
    //     //     macroquad::color::Color::new(1., 1., 1., 1.0),
    //     // );
    //     // draw_line(
    //     //     (viewport.offset().x + center.x) * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y + center.y + viewport.get().h)
    //     //         * texture_manager.cell_output_size().y,
    //     //     (viewport.offset().x + viewport.get().w + center.x)
    //     //         * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y + center.y + viewport.get().h)
    //     //         * texture_manager.cell_output_size().y,
    //     //     10.,
    //     //     macroquad::color::Color::new(1., 1., 1., 1.0),
    //     // );

    //     // draw_line(
    //     //     (viewport.offset().x + viewport.get().w + center.x)
    //     //         * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y + center.y) * texture_manager.cell_output_size().y,
    //     //     (viewport.offset().x + center.x) * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y + center.y + viewport.get().h)
    //     //         * texture_manager.cell_output_size().y,
    //     //     10.,
    //     //     macroquad::color::Color::new(1., 1., 1., 1.0),
    //     // );

    //     // draw_line(
    //     //     (viewport.offset().x + center.x) * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y + center.y + viewport.get().h)
    //     //         * texture_manager.cell_output_size().y,
    //     //     (viewport.offset().x + center.x) * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y + center.y) * texture_manager.cell_output_size().y,
    //     //     10.,
    //     //     macroquad::color::Color::new(1., 1., 1., 1.0),
    //     // );

    //     // print line points
    //     println!("viewport: {:?}", viewport.get());
    //     // println!("center: {:?}", center);
    //     // println!("offset: {:?}", viewport.offset());
    //     // println!(
    //     //     "start x: {}, start y: {}",
    //     //     (viewport.offset().x) * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y) * texture_manager.cell_output_size().y
    //     // );
    //     // println!(
    //     //     "end x: {}, end y: {}",
    //     //     (viewport.offset().x + viewport.get().w) * texture_manager.cell_output_size().x,
    //     //     (viewport.offset().y) * texture_manager.cell_output_size().y
    //     // );
    // }
}
