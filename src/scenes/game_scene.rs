use macroquad::{
    prelude::{debug, info, WHITE},
    text::draw_text,
    window::{screen_height, screen_width},
};

use super::{events::SceneEvent, Scene, SceneContext, UpdatableScene};

pub struct GameScene {
    log: Vec<String>,
}

impl GameScene {
    pub fn new() -> Self {
        Self { log: vec![] }
    }

    pub fn log(&self) -> &Vec<String> {
        &self.log
    }

    pub fn add_log(&mut self, log: String) {
        if self.log.len() > 10 {
            self.log.remove(0);
        }
        self.log.push(log);
    }
}

impl UpdatableScene for GameScene {
    fn update(&mut self) {
        debug!("GameScene update");
    }

    fn draw(&self) {
        debug!("GameScene draw");
        draw_text(
            "GAME!!!!!",
            screen_width() / 2.,
            screen_height() / 2.,
            120.,
            WHITE,
        );

        let mut y = 50.;

        for log in self.log.iter() {
            draw_text(log, screen_width() - 200., y, 20., WHITE);
            y += 20.;
        }
    }
}

impl nefsm::sync::Stateful<Scene, SceneContext, SceneEvent> for GameScene {
    fn on_enter(&mut self, _context: &mut SceneContext) -> nefsm::sync::Response<Scene> {
        println!("GameScene state on enter");

        nefsm::sync::Response::Handled
    }

    fn on_event(
        &mut self,
        event: &SceneEvent,
        _context: &mut SceneContext,
    ) -> nefsm::sync::Response<Scene> {
        println!("GameScene state on event : {:?}", event);
        match event {
            SceneEvent::Update => {
                self.update();
                nefsm::sync::Response::Handled
            }
            SceneEvent::Draw => {
                self.draw();
                nefsm::sync::Response::Handled
            }
            SceneEvent::Mouse(mouse_events) => {
                info!("MouseEvents : {:?}", mouse_events);

                self.add_log(format!("MouseEvents : {:?}", mouse_events));

                nefsm::sync::Response::Handled
            }
            SceneEvent::KeyPressed(key) => {
                info!("KeyPressedEvent : {:?}", key);

                nefsm::sync::Response::Handled
            }
            _ => nefsm::sync::Response::Handled,
        }
    }

    fn on_exit(&mut self, _context: &mut SceneContext) {
        println!("GameScene state on exit");
    }
}
