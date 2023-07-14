use std::{cell::RefCell, rc::Rc};

use crate::engine::core::Engine;

pub mod end_scene;
pub mod events;
pub mod fsm;
pub mod game_scene;
pub mod intro_scene;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Scene {
    Intro,
    Game,
    End,
}

pub trait UpdatableScene {
    fn update(&mut self) {}
    fn draw(&self) {}
}

#[derive(Debug)]
pub struct SceneContext {
    pub game: Engine,
}
