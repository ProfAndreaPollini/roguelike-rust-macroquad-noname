use crate::player::Player;

pub enum Action {
    Move(i32, i32),
}

pub struct ActionHandler {
    actions: Vec<Action>,
}

pub trait Movable {
    fn move_by(&mut self, x: i32, y: i32);
}

impl ActionHandler {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn handle_actions(&mut self, player: &mut Player) {
        for action in self.actions.iter() {
            match action {
                Action::Move(x, y) => {
                    println!("Move to {}, {}", x, y);
                    player.x += x;
                    player.y += y;
                }
            }
        }
        self.actions.clear();
    }
}
