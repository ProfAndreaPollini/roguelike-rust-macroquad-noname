use crate::engine::{
    core::world::{EntityKey, World},
    map::Map,
};

#[derive(Debug)]
pub enum ActionResult {
    Succeeded,
    Failure,
    AlternativeAction(Action),
}

#[derive(Debug)]
pub struct Move {
    pub dx: i32,
    pub dy: i32,

    pub key: EntityKey,
}

#[derive(Debug)]
pub enum Action {
    Move(Move),
}

impl Action {
    pub fn perform(&self, map: &mut Map, world: &mut World) -> ActionResult {
        match self {
            Action::Move(Move { dx, dy, key }) => {
                let e = world.get_entity_mut(*key).unwrap();

                let pos = e.position();

                if pos.is_none() {
                    return ActionResult::Failure;
                }
                let (x, y) = pos.unwrap();
                let desiderd_x = x + dx;
                let desiderd_y = y + dy;

                println!("Desired position: {}, {}", desiderd_x, desiderd_y);

                if !map.is_valid_position(e, desiderd_x, desiderd_y) {
                    return ActionResult::Failure;
                }

                e.move_by(*dx, *dy);
                match (dx.signum(), dy.signum()) {
                    (1, 0) => e.set_direction(crate::engine::core::direction::Direction::Right),
                    (-1, 0) => e.set_direction(crate::engine::core::direction::Direction::Left),
                    (0, 1) => e.set_direction(crate::engine::core::direction::Direction::Down),
                    (0, -1) => e.set_direction(crate::engine::core::direction::Direction::Up),
                    _ => {}
                }

                ActionResult::Succeeded
            }
            _ => ActionResult::Failure,
        };

        ActionResult::Failure
    }
}
