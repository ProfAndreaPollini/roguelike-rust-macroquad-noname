use macroquad::{
    prelude::WHITE,
    text::{draw_text_ex, TextParams},
    window::{screen_height, screen_width},
};

use super::UpdatableScene;

pub struct IntroScene {}

impl UpdatableScene for IntroScene {
    fn process_input(
        &mut self,
        event: super::events::SceneEvent,
    ) -> Option<super::events::SceneCommands> {
        match event {
            super::events::SceneEvent::KeyPressed(key) => {
                if key == macroquad::input::KeyCode::Space {
                    Some(super::events::SceneCommands::ChangeScene(
                        super::Scene::Game,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn update(&mut self) {
        // info!("IntroScene update");
    }

    fn draw(&self) {
        // info!("IntroScene draw");
        draw_text_ex(
            "Rust Roguelike",
            screen_width() / 2.,
            screen_height() / 2.,
            {
                TextParams {
                    font_size: 120,
                    font_scale: 1.0,
                    color: WHITE,
                    ..TextParams::default()
                }
            },
        );
    }
}
