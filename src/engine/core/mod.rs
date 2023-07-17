#![allow(dead_code)]

use crate::{
    actions::{Action, ActionHandler, ActionResult},
    engine::map::Map,
};

use macroquad::prelude::{IVec2, Vec2};

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use self::entity::{Entity, EntityFeatures};

use super::{fov::compute_fov, texture_manager::TextureManager, viewport::Viewport};

pub mod entity;

#[derive(Debug, Clone)]
pub struct Engine(Rc<RefCell<EngineRepr>>);

impl Engine {
    pub fn new(texture_manager: TextureManager, map: Map) -> Self {
        Self(Rc::new(RefCell::new(EngineRepr::new(texture_manager, map))))
    }

    pub fn update(&self) {
        // RefCell::borrow_mut(&self.0).update(); //.update();
        let mut action: Option<Action> = None;

        let engine_mut = &mut *self.0.borrow_mut();

        let current_entity = {
            let current_entity = { engine_mut.current_entity };
            println!("Current entity: {}", current_entity);
            println!("NPC count: {}", engine_mut.entities.len());
            if current_entity >= engine_mut.entities.len() - 1 {
                engine_mut.current_entity = 0;
            } else {
                engine_mut.current_entity += 1;
            }
            engine_mut.current_entity
        };
        let mut current = { &mut engine_mut.entities[current_entity] };

        let mut actions: Vec<Action> = vec![];

        let next_action = current.next_action();

        match next_action {
            Some(action) => {
                actions.push(action);
            }
            None => return,
        }

        while let Some(action) = actions.pop() {
            let action_reponse = action.perform(current, &mut engine_mut.map);
            match action_reponse {
                ActionResult::Succeeded => {}
                ActionResult::Failure => {}
                ActionResult::AlternativeAction(action) => {
                    actions.push(action);
                }
            }
            // }
        }
    }

    pub fn entity_at(&self, pos: usize) -> Entity {
        self.0.borrow().entities[pos].clone()
    }

    pub fn npc_count(&self) -> usize {
        self.0.borrow().entities.len()
    }

    pub fn render(&self) {
        self.0.borrow().render();
    }

    pub fn update_fov(&self) {
        RefCell::borrow_mut(&self.0).update_fov();
    }

    pub fn current_entity(&self) -> usize {
        self.0.borrow().current_entity
    }

    pub fn map(&self) -> Ref<Map> {
        Ref::map(self.0.borrow(), |x| &x.map)
    }

    pub fn action_handler(&self) -> Ref<ActionHandler> {
        Ref::map(self.0.borrow(), |x| &x.action_handler)
    }

    pub fn viewport(&self) -> Ref<Viewport> {
        Ref::map(self.0.borrow(), |x| &x.viewport)
    }

    pub fn viewport_m(&self) -> RefMut<Viewport> {
        RefMut::map(self.0.borrow_mut(), |x: &mut EngineRepr| &mut x.viewport)
    }
}

#[derive(Debug)]
pub struct EngineRepr {
    // pub player: Player,
    texture_manager: TextureManager,
    pub action_handler: ActionHandler,
    pub map: Map,
    // pub npc_list: Vec<NPC>,
    entities: Vec<Entity>,
    current_entity: usize,
    pub viewport: Viewport,
    // current_entity: Box<dyn Entity>,
}

impl EngineRepr {
    pub fn new(texture_manager: TextureManager, map: Map) -> Self {
        // let player = Player::new();
        // player.add_sprite(&texture_manager, "idle", 17, 0);
        let action_handler = ActionHandler::new();
        // let npc = NPC::new(15, 15, "npc01".to_string());
        let current_entity = usize::MAX;
        let mut player = Entity::Player(EntityFeatures::new("player".to_string()));

        player.move_to(15, 15);

        Self {
            // current_entity: Box::new(player),
            // player,
            texture_manager,
            action_handler,
            map,
            // npc_list: vec![npc],
            viewport: Viewport::new(0.0, 0.0, 40.0, 30.0, Vec2::new(17.5, 18.7)),
            current_entity,
            entities: vec![
                player,
                Entity::NPC(EntityFeatures::new("npc01".to_string())),
            ],
        }
    }

    pub fn current_entity(&self) -> &Entity {
        &self.entities[self.current_entity]
    }

    pub fn current_entity_mut(&mut self) -> &mut Entity {
        &mut self.entities[self.current_entity]
    }

    pub fn render(&self) {
        self.map.draw(&self.texture_manager, &self.viewport);

        self.entities[0].draw(&self.texture_manager, &self.viewport);

        // get entities from index 1 to the end
        self.entities[1..]
            .iter()
            .for_each(|x| x.draw(&self.texture_manager, &self.viewport));
    }

    pub fn update_fov(&mut self) {
        let fov_distance: i32 = 5;

        self.map.set_all_tiles_visibility(false);

        let position = self.entities[0].position().unwrap();

        compute_fov(
            &mut self.map,
            IVec2::new(position.0, position.1),
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
