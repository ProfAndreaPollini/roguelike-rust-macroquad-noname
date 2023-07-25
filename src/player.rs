use macroquad::{
    prelude::{is_key_pressed, KeyCode, Vec2, YELLOW},
    shapes::draw_line,
};

use crate::{
    actions::{Action, Move},
    engine::{
        core::{
            camera::Camera,
            direction::Direction,
            entity::{Drawable, Updatable},
            world::{EntityKey, World},
        },
        map::{renderer::render_entity, Map},
        texture_manager::TextureManager,
    },
};

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

impl Drawable for Player {
    fn draw(&self, texture_manager: &TextureManager, camera: &Camera) {
        // println!("Player draw");
        let texture = &texture_manager.texture;
        let sprite_rect = texture_manager.get_sprite("idle");
        let cell_size = texture_manager.cell_size;

        let sprite_x = self.x as f32 * cell_size;
        let sprite_y = self.y as f32 * cell_size;
        let screen_pos = camera.world_to_viewport(Vec2::new(sprite_x, sprite_y));

        let final_pos = match self.direction {
            Direction::Down => screen_pos + Vec2::new(0., cell_size),
            Direction::Up => screen_pos + Vec2::new(0., -cell_size),
            Direction::Left => screen_pos + Vec2::new(-cell_size, 0.),
            Direction::Right => screen_pos + Vec2::new(cell_size, 0.),
        };

        render_entity(
            Vec2::new(self.x as f32, self.y as f32),
            sprite_rect,
            texture,
            cell_size,
            camera,
        );
        draw_line(
            screen_pos.x + cell_size / 2.,
            screen_pos.y + cell_size / 2.,
            final_pos.x + cell_size / 2.,
            final_pos.y + cell_size / 2.,
            2.,
            YELLOW,
        );
    }
}

impl Updatable for Player {
    fn update(&self, map: &mut Map, world: &World, key: EntityKey) -> Vec<Action> {
        // println!("Player update");

        let mut actions: Vec<Action> = vec![];

        let next_action = self.next_action(map, world, key);

        if let Some(action) = next_action {
            actions.push(action);
        }

        // while let Some(action) = actions.pop() {
        //     // let action_reponse = action.perform(map);
        //     match action_reponse {
        //         ActionResult::Succeeded => {}
        //         ActionResult::Failure => {}
        //         ActionResult::AlternativeAction(action) => {
        //             actions.push(action);
        //         }
        //     }
        // }
        actions
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn next_action(
        &self,
        map: &Map,
        world: &World,
        key: EntityKey,
    ) -> Option<crate::actions::Action> {
        let mut action = None;

        if is_key_pressed(KeyCode::Right) {
            // self.x += 1;
            action = Some(crate::actions::Action::Move(Move { dx: 1, dy: 0, key }));
        }
        if is_key_pressed(KeyCode::Left) {
            // self.x -= 1;
            //action_handler.add_action(crate::actions::Action::Move(Move { dx: -1, dy: 0 }));
            action = Some(crate::actions::Action::Move(Move { dx: -1, dy: 0, key }));
        }
        if is_key_pressed(KeyCode::Up) {
            // self.y -= 1;
            //action_handler.add_action(crate::actions::Action::Move(Move { dx: 0, dy: -1 }));
            action = Some(crate::actions::Action::Move(Move { dx: 0, dy: -1, key }));
        }
        if is_key_pressed(KeyCode::Down) {
            // self.y += 1;
            //action_handler.add_action(crate::actions::Action::Move(Move { dx: 0, dy: 1 }));
            action = Some(crate::actions::Action::Move(Move { dx: 0, dy: 1, key }));
        }
        action
    }

    fn position(&self) -> Option<(i32, i32)> {
        Some((self.x, self.y))
    }
}

// impl Player {
//     pub fn new() -> Self {
//         Self {
//             x: 12,
//             y: 12,
//             ..Default::default()
//         }
//     }
// }
