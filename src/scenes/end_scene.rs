use super::{events::SceneEvent, Scene, SceneContext};

pub struct EndScene {}

impl nefsm::sync::Stateful<Scene, SceneContext, SceneEvent> for EndScene {
    fn on_enter(&mut self, _context: &mut SceneContext) -> nefsm::sync::Response<Scene> {
        println!("Ready state on enter");
        nefsm::sync::Response::Handled
    }

    fn on_event(
        &mut self,
        event: &SceneEvent,
        _context: &mut SceneContext,
    ) -> nefsm::sync::Response<Scene> {
        println!("Ready state on event : {:?}", event);
        match event {
            SceneEvent::EndGame => nefsm::sync::Response::Transition(Scene::Intro),
            _ => nefsm::sync::Response::Handled,
        }
    }

    fn on_exit(&mut self, _context: &mut SceneContext) {
        println!("Ready state on exit");
    }
}
