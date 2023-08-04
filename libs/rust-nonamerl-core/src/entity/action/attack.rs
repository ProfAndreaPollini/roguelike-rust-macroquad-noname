#![allow(dead_code)]
use crate::{
    entity::world::{EntityKey, World},
    Map, Tile,
};

use super::Action;
#[derive(Debug)]
pub struct AttackAction<T: Tile> {
    pub damage: i32,
    pub target: EntityKey,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Tile> AttackAction<T> {
    pub fn new(damage: i32) -> Self {
        Self {
            damage,
            target: Default::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn to(mut self, target: EntityKey) -> Self {
        self.target = target;
        self
    }
}

impl<T: Tile> Action<T> for AttackAction<T> {
    fn perform(&self, world: &World<T>, _map: &mut Map<T>) {
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
