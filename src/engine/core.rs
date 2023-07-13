use crate::{
    actions::{handle_actions, ActionHandler},
    engine::map::Map,
    player::Player,
};

use macroquad::prelude::IVec2;
use std::rc::Rc;
use std::{borrow::BorrowMut, cell::RefCell};

use super::{fov::compute_fov, texture_manager::TextureManager};

pub trait Drawable {
    fn draw(&self, texture_manager: &TextureManager);
}

pub trait Entity {
    fn x(&mut self) -> &mut i32;
    fn y(&mut self) -> &mut i32;

    fn set_x(&mut self, x: i32) {
        *self.x() = x;
    }

    fn set_y(&mut self, y: i32) {
        *self.y() = y;
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        *self.x() += dx;
        *self.y() += dy;
    }

    fn as_player_mut(&mut self) -> Option<&mut Player> {
        None
    }
}

pub struct Engine {
    pub player: Rc<RefCell<Player>>,
    texture_manager: TextureManager,
    pub action_handler: ActionHandler,
    pub map: Map,
    // current_entity: Box<dyn Entity>,
}

impl Engine {
    pub fn new(texture_manager: TextureManager, map: Map) -> Self {
        let player = Player::new();
        // player.add_sprite(&texture_manager, "idle", 17, 0);
        let action_handler = ActionHandler::new();

        Self {
            // current_entity: Box::new(player),
            player: Rc::new(RefCell::new(player)),
            texture_manager,
            action_handler,
            map,
        }
    }

    // pub fn player(&mut self) -> Rc<RefCell<Player>> {
    //     self.player.clone()
    // }

    pub fn update(&mut self) {
        self.handle_input();
        handle_actions(self);
    }

    pub fn handle_input(&mut self) {
        let binding = self.player.borrow_mut();
        let x = binding.as_ref();
        x.borrow_mut().handle_input(&mut self.action_handler);
        //.handle_input(&mut self.action_handler);
        //.handle_input(&mut self.action_handler);
    }

    pub fn render(&self) {
        self.map.draw(&self.texture_manager);
        self.player.borrow().draw(&self.texture_manager);
    }

    pub fn update_fov(&mut self) {
        let fov_distance: i32 = 5;

        // self.map.set_tile_range_visibility(
        //     self.player.borrow().x as u32,
        //     self.player.borrow().y as u32,
        //     FOV_DISTANCE as u32,
        //     false,
        // );

        self.map.set_all_tiles_visibility(false);

        compute_fov(
            &mut self.map,
            IVec2::new(self.player.borrow().x, self.player.borrow().y),
            fov_distance,
        );

        // let mut fov_coords = vec![];

        // for i in 0..8 {
        //     let mut octant_coords = self.fov_octant(start_pos, FOV_DISTANCE + 1, i);
        //     fov_coords.append(&mut octant_coords);
        // }
        // // self.fov_octant(start_pos, FOV_DISTANCE, 0);

        // fov_coords.iter().for_each(|c| {
        //     let map = self.map.borrow_mut();
        //     map.set_tile_visible(c.x as u16, c.y as u16, true)
        // });
    }
}
