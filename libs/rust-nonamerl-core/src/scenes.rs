use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct SceneContext {}

pub enum SceneCommands {
    AddScene(Box<dyn Scene>),
    PopScene,
    PushScene(usize),
    ReplaceScene(usize),
    Exit,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPressed,
    KeyReleased,
    MouseMoved,
    MousePressed,
    MouseReleased,
}

#[derive(Debug, Clone)]
pub enum Events {
    InputEvent(InputEvent),
}

pub enum Commands {
    SceneCommand(SceneCommands),
}

pub trait Scene {
    fn process_input(&mut self, event: InputEvent) -> Option<SceneCommands> {
        None
    }
    fn setup(&mut self, context: Rc<SceneContext>) {}
    fn update(&mut self) {}
    fn draw(&self) {}
    fn draw_ui(&mut self) {}
}

#[derive(Default)]
pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    scene_stack: Vec<usize>,
}

type Type = Box<dyn Scene>;

impl SceneManager {
    pub fn current_scene(&self) -> Option<&Type> {
        self.scene_stack
            .last()
            .map(move |scene_index| &self.scenes[*scene_index])
    }

    pub fn current_scene_mut(&mut self) -> Option<&mut Type> {
        self.scene_stack
            .last()
            .map(|scene_index| &mut self.scenes[*scene_index])
    }

    pub fn process_command(&mut self, command: SceneCommands) {
        match command {
            SceneCommands::AddScene(scene) => {
                self.scenes.push(scene);
            }
            SceneCommands::PushScene(scene) => {
                self.scene_stack.push(scene);
            }
            SceneCommands::PopScene => {
                self.scene_stack.pop();
            }
            SceneCommands::ReplaceScene(scene) => {
                self.scene_stack.pop();
                self.scene_stack.push(scene);
            }
            SceneCommands::Exit => {}
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    struct TestScene {}

    impl Scene for TestScene {}

    struct TestScene2 {}

    impl Scene for TestScene2 {}

    #[test]
    fn test_scene_manager() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        scene_manager.process_command(SceneCommands::PushScene(0));
        assert!(scene_manager.current_scene().is_some());
    }

    #[test]
    fn test_scene_manager_pop() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        scene_manager.process_command(SceneCommands::PushScene(0));
        scene_manager.process_command(SceneCommands::PopScene);
        assert!(scene_manager.current_scene().is_none());
    }

    #[test]

    fn test_scene_manager_replace() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        scene_manager.process_command(SceneCommands::PushScene(0));
        let scene2 = Box::new(TestScene2 {});
        scene_manager.process_command(SceneCommands::AddScene(scene2));
        scene_manager.process_command(SceneCommands::ReplaceScene(1));
        assert!(scene_manager.current_scene().is_some());
    }

    #[test]
    fn test_scene_manager_process_input() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        scene_manager.process_command(SceneCommands::PushScene(0));
        let command = scene_manager
            .current_scene_mut()
            .unwrap()
            .process_input(InputEvent::KeyPressed);
        assert!(command.is_none());
    }

    #[test]
    fn test_scene_manager_setup() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        scene_manager.process_command(SceneCommands::PushScene(0));
        scene_manager
            .current_scene_mut()
            .unwrap()
            .setup(Rc::new(SceneContext {}));
    }

    #[test]
    fn test_scene_manager_update() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        scene_manager.process_command(SceneCommands::PushScene(0));
        scene_manager.current_scene_mut().unwrap().update();
    }

    #[test]
    fn test_scene_manager_draw() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        scene_manager.process_command(SceneCommands::PushScene(0));
        scene_manager.current_scene().unwrap().draw();
    }

    #[test]
    fn test_scene_manager_draw_ui() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        scene_manager.process_command(SceneCommands::PushScene(0));
        scene_manager.current_scene_mut().unwrap().draw_ui();
    }

    #[test]
    fn test_scene_manager_change_scene() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        let scene2 = Box::new(TestScene2 {});
        scene_manager.process_command(SceneCommands::AddScene(scene2));
        scene_manager.process_command(SceneCommands::PushScene(0));
        scene_manager.process_command(SceneCommands::PushScene(1));
        assert!(scene_manager.current_scene().is_some());
    }

    #[test]
    fn test_cycle_through_scenes() {
        let mut scene_manager = SceneManager::default();
        let scene = Box::new(TestScene {});
        scene_manager.process_command(SceneCommands::AddScene(scene));
        let scene2 = Box::new(TestScene2 {});
        scene_manager.process_command(SceneCommands::AddScene(scene2));
        scene_manager.process_command(SceneCommands::PushScene(0));
        assert_eq!(scene_manager.scene_stack.len(), 1);
        assert_eq!(*scene_manager.scene_stack.first().unwrap(), 0);
        scene_manager.process_command(SceneCommands::PushScene(1));
        scene_manager.process_command(SceneCommands::PopScene);
        assert!(scene_manager.current_scene().is_some());
    }
}
