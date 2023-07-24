#![allow(dead_code)]

use crate::engine::map::Map;

use macroquad::prelude::{IVec2, Vec2};

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use self::entity::{Entity, EntityFeatures};

use super::{fov::compute_fov, texture_manager::TextureManager, viewport::Viewport};

pub mod camera;
pub mod entity;
pub mod world;

#[derive(Debug, Clone)]
pub struct Engine(Rc<RefCell<EngineRepr>>);

impl Engine {
    pub fn new(texture_manager: TextureManager, map: Map) -> Self {
        Self(Rc::new(RefCell::new(EngineRepr::new(texture_manager, map))))
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

    pub fn viewport(&self) -> Ref<Viewport> {
        Ref::map(self.0.borrow(), |x| &x.viewport)
    }

    pub fn viewport_m(&self) -> RefMut<Viewport> {
        RefMut::map(self.0.borrow_mut(), |x: &mut EngineRepr| &mut x.viewport)
    }
}

#[derive(Debug)]
pub struct EngineRepr {
    texture_manager: TextureManager,

    pub map: Map,

    entities: Vec<Entity>,
    current_entity: usize,
    pub viewport: Viewport,
}

impl EngineRepr {
    pub fn new(texture_manager: TextureManager, map: Map) -> Self {
        let current_entity = usize::MAX;
        let mut player = Entity::Player(EntityFeatures::new("player".to_string()));

        player.move_to(15, 15);

        Self {
            texture_manager,

            map,

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

        // self.entities[0].draw(&self.texture_manager, &self.viewport);

        // get entities from index 1 to the end
        // self.entities[1..]
        //     .iter()
        //     .for_each(|x| x.draw(&self.texture_manager, &self.viewport));
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
    }
}
