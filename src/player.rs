use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{
    actions::{Action, ActionResult, Move},
    engine::{
        core::{
            entity::{draw_sprite, Drawable, Updatable},
            world::{EntityKey, World},
        },
        map::Map,
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
    fn update(&self, map: &mut Map, world: &World, key: EntityKey) -> Vec<Action> {
        println!("Player update");

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

impl Player {
    pub fn new() -> Self {
        Self { x: 12, y: 12 }
    }
}
