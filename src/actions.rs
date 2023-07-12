use std::cell::RefCell;

use crate::engine::core::{Engine, Entity};

pub trait Performable<'a> {
    fn perform(&self, entity: &'a RefCell<dyn Entity>, engine: &mut Engine);
}

#[derive(Clone)]
pub struct Move {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Clone)]
pub enum Action {
    Move(Move),
}

impl<'a> Performable<'a> for Action {
    fn perform(&self, entity: &'a RefCell<dyn Entity>, engine: &mut Engine) {
        let map = &mut engine.map;

        let mut binding = entity.borrow_mut();
        let e = binding.as_player_mut().unwrap();

        //let entity = e.as_player_mut().unwrap();
        match self {
            Action::Move(Move { dx: x, dy: y }) => {
                // get desired position
                let desired_x = *e.x() + x;
                let desired_y = *e.y() + y;

                // println!("Desired position: {}, {}", desired_x, desired_y);

                // check if position is valid
                if !map.is_valid_position(e, desired_x, desired_y) {
                    return;
                }

                // println!("Move to {}, {}", x, y);
                // *entity.x() += x;
                // *entity.y() += y;
                //e.set_x(desired_x);
                *(e.x()) += x;
                *(e.y()) += y;

                // let x = *e.x();
                // let y = *e.y();
                // // print current position
                // // println!("Current position: {}, {}", x, y);
            }
        }
    }
}

pub struct ActionHandler {
    actions: Vec<Action>,
}

pub trait Movable {
    fn move_by(&mut self, x: i32, y: i32);
}

pub fn handle_actions(engine: &mut Engine) {
    // let action_handler = engine.action_handler;
    // let entity = Box::new(engine.player().clone()) as Box<dyn Entity>;
    let actions = engine.action_handler.actions.clone();
    // println!("{:?}", engine.player.borrow());
    let p = engine.player.clone();
    // println!("{:?}", p);
    // p.as_ref().borrow_mut().move_by(1, 0); // * p.borrow_mut().x() += 1;
    // let map = engine.map.clone();

    for action in actions.iter() {
        action.perform(p.as_ref(), engine);
    }

    // actions.iter().for_each(|action| {
    //     action.perform(entity, &mut engine.map);
    // });
    engine.action_handler.actions.clear();
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
}
