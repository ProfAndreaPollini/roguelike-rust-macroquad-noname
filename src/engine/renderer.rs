// use std::{cell::RefCell, rc::Rc};

// use crate::actions::{Action, ActionHandler};

// use super::{texture_manager::TextureManager, viewport::Viewport};

// #[derive(Debug)]
// struct Renderer(Rc<RefCell<RendererRepr>>);

// #[derive(Debug)]
// struct RendererRepr {
//     texture_manager: TextureManager,
//     viewport: Viewport,
// }

// struct Player {
//     x: i32,
//     y: i32,
// }

// impl Player {
//     fn new() -> Self {
//         Self { x: 0, y: 0 }
//     }
// }

// impl Entity for Player {
//     fn update(&mut self) {}
//     fn position(&self) -> (i32, i32) {
//         (self.x, self.y)
//     }
//     fn is_player(&self) -> bool {
//         true
//     }
// }

// struct NPC {
//     x: i32,
//     y: i32,
//     //brain: Brain,
// }

// impl NPC {
//     fn new(x: i32, y: i32) -> Self {
//         Self { x, y }
//     }
// }

// impl Entity for NPC {
//     fn position(&self) -> (i32, i32) {
//         (self.x, self.y)
//     }
// }
