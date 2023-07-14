use nefsm::sync::{EventHandler, FsmEnum, Response, Stateful};

use super::{
    end_scene::EndScene, events::SceneEvent, game_scene::GameScene, intro_scene::IntroScene, Scene,
    SceneContext,
};

impl FsmEnum<Scene, SceneContext, SceneEvent> for Scene {
    fn create(enum_value: &Scene) -> Box<dyn Stateful<Scene, SceneContext, SceneEvent> + Send> {
        match enum_value {
            Scene::Intro => Box::new(IntroScene {}),
            Scene::Game => Box::new(GameScene {}),
            Scene::End => Box::new(EndScene {}),
        }
    }
}

pub struct GlobalStateTransitionHandler;

impl EventHandler<Scene, SceneContext, SceneEvent> for GlobalStateTransitionHandler {
    fn on_event(&mut self, _event: &SceneEvent, _context: &mut SceneContext) -> Response<Scene> {
        Response::Handled
    }
}
