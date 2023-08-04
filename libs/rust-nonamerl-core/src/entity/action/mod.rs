use crate::{Map, Tile};

use super::world::World;

pub trait Action<T: Tile> {
    fn perform(&self, world: &World<T>, map: &mut Map<T>);
}

#[derive(Debug)]
pub struct DummyAction {}

impl<T: Tile> Action<T> for DummyAction {
    fn perform(&self, _world: &World<T>, _map: &mut Map<T>) {
        println!("Dummy action");
    }
}

pub mod attack;
pub mod equip;
pub mod move_entity;
pub mod queue;

pub use attack::AttackAction;
pub use equip::EquipAction;
pub use move_entity::MoveAction;
pub use queue::ActionQueue;
