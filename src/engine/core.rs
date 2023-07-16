#![allow(dead_code)]

use crate::{
    actions::{handle_actions, ActionHandler},
    engine::map::Map,
    npc::NPC,
    player::Player,
};

use macroquad::prelude::{IVec2, Vec2};

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::{fov::compute_fov, texture_manager::TextureManager, viewport::Viewport};

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

#[derive(Debug, Clone)]
pub struct Engine(Rc<RefCell<EngineRepr>>);

impl Engine {
    pub fn new(texture_manager: TextureManager, map: Map) -> Self {
        Self(Rc::new(RefCell::new(EngineRepr::new(texture_manager, map))))
    }

    pub fn update(&self) {
        RefCell::borrow_mut(&self.0).update(); //.update();
    }

    pub fn render(&self) {
        self.0.borrow().render();
    }

    pub fn update_fov(&self) {
        RefCell::borrow_mut(&self.0).update_fov();
    }

    pub fn map(&self) -> Ref<Map> {
        Ref::map(self.0.borrow(), |x| &x.map)
    }

    pub fn action_handler(&self) -> Ref<ActionHandler> {
        Ref::map(self.0.borrow(), |x| &x.action_handler)
    }

    pub fn player(&self) -> Rc<RefCell<Player>> {
        self.0.borrow().player.clone()
    }

    // pub fn get_mut(&self) -> RefCell<EngineRepr> {
    //     self.0.borrow_mut()
    // }
    pub fn viewport(&self) -> Ref<Viewport> {
        Ref::map(self.0.borrow(), |x| &x.viewport)
    }

    pub fn viewport_mut(&self) -> RefMut<Viewport> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.viewport)
    }
}

#[derive(Debug)]
pub struct EngineRepr {
    pub player: Rc<RefCell<Player>>,
    texture_manager: TextureManager,
    pub action_handler: ActionHandler,
    pub map: Map,
    pub npc_list: Vec<NPC>,
    pub viewport: Viewport,
    // current_entity: Box<dyn Entity>,
}

impl EngineRepr {
    pub fn new(texture_manager: TextureManager, map: Map) -> Self {
        let player = Player::new();
        // player.add_sprite(&texture_manager, "idle", 17, 0);
        let action_handler = ActionHandler::new();
        let npc = NPC::new(15, 15, "npc01".to_string());

        Self {
            // current_entity: Box::new(player),
            player: Rc::new(RefCell::new(player)),
            texture_manager,
            action_handler,
            map,
            npc_list: vec![npc],
            viewport: Viewport::new(0.0, 0.0, 40.0, 30.0, Vec2::new(17.5, 18.7)),
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
        let mut binding = self.player.borrow_mut();
        let x = binding.as_player_mut().unwrap();
        x.handle_input(&mut self.action_handler);
        //.handle_input(&mut self.action_handler);
        //.handle_input(&mut self.action_handler);
    }

    pub fn render(&self) {
        self.map.draw(&self.texture_manager, &self.viewport);

        self.player
            .borrow()
            .draw(&self.texture_manager, &self.viewport);
        self.npc_list
            .iter()
            .for_each(|npc| npc.draw(&self.texture_manager));
    }

    pub fn update_fov(&mut self) {
        let fov_distance: i32 = 5;

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
