use std::rc::Rc;

use crate::engine::{core::Engine, texture_manager::TextureManager};

use self::events::SceneCommands;

pub mod end_scene;
pub mod events;
pub mod game_scene;
pub mod intro_scene;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Scene {
    Intro,
    Game,
    End,
}

pub trait UpdatableScene {
    fn process_input(&mut self, event: events::SceneEvent) -> Option<SceneCommands> {
        None
    }
    fn setup(&mut self, context: Rc<SceneContext>) {}
    fn update(&mut self) {}
    fn draw(&self) {}
}

#[derive(Debug, Default)]
pub struct SceneContext {
    pub texture_manager: Option<Rc<TextureManager>>,
}
