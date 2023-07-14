use macroquad::{
    prelude::{info, KeyCode, WHITE},
    text::draw_text,
    window::{screen_height, screen_width},
};

use super::{events::SceneEvent, Scene, SceneContext, UpdatableScene};

pub struct IntroScene {}

impl UpdatableScene for IntroScene {
    fn update(&mut self) {
        // info!("IntroScene update");
    }

    fn draw(&self) {
        // info!("IntroScene draw");
        draw_text(
            "Rust Roguelike",
            screen_width() / 2.,
            screen_height() / 2.,
            72.,
            WHITE,
        );
    }
}

impl nefsm::sync::Stateful<Scene, SceneContext, SceneEvent> for IntroScene {
    fn on_enter(&mut self, _context: &mut SceneContext) -> nefsm::sync::Response<Scene> {
        // println!("Null state on enter, retries = {}", context.retries);
        // nefsm::sync::Response::Transition(Scene::Game)
        info!("Intro state on enter");
        nefsm::sync::Response::Handled
    }

    fn on_event(
        &mut self,
        event: &SceneEvent,
        _context: &mut SceneContext,
    ) -> nefsm::sync::Response<Scene> {
        match event {
            SceneEvent::Update => {
                info!("Intro state on event : {:?}", event);
                self.update();
                nefsm::sync::Response::Handled
            }
            SceneEvent::Draw => {
                info!("Intro state on event : {:?}", event);
                self.draw();
                nefsm::sync::Response::Handled
            }
            SceneEvent::KeyPressed(key) => {
                info!("KeyPressedEvent : {:?}", key);
                if *key == KeyCode::Space {
                    nefsm::sync::Response::Transition(Scene::Game)
                } else {
                    nefsm::sync::Response::Handled
                }
            }
            _ => nefsm::sync::Response::Handled,
        }
    }

    fn on_exit(&mut self, _context: &mut SceneContext) {
        println!("Null state on exit");
    }
}
