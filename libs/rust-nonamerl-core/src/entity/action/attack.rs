#![allow(dead_code)]
use crate::entity::world::{EntityKey, World};

use super::Action;
#[derive(Debug)]
pub struct AttackAction {
    pub damage: i32,
    pub target: EntityKey,
}

impl AttackAction {
    pub fn new(damage: i32) -> Self {
        Self {
            damage,
            target: Default::default(),
        }
    }

    pub fn to(mut self, target: EntityKey) -> Self {
        self.target = target;
        self
    }
}

impl Action for AttackAction {
    fn perform(&self, world: &World) {
        if self.target == Default::default() {
            panic!("Target not set");
        }
        let entities = world.entities.borrow();
        let target = entities.get(self.target).unwrap();
        println!(
            "Attack action to target {:?} inflicting {} damage",
            target, self.damage
        );
    }
}
