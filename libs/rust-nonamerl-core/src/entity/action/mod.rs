use super::world::World;

pub trait Action {
    fn perform(&self, world: &World);
}

#[derive(Debug)]
pub struct DummyAction {}

impl Action for DummyAction {
    fn perform(&self, _world: &World) {
        println!("Dummy action");
    }
}

pub mod attack;
pub mod equip;
pub mod move_entity;

pub use attack::AttackAction;
pub use equip::EquipAction;
pub use move_entity::MoveAction;
