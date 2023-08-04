#![allow(dead_code)]
use crate::{
    entity::world::{EntityKey, World},
    Map, Tile,
};

use super::Action;

/// This module contains the implementation of the `Attack` action for entities in the game.
/// The `Attack` action allows an entity to attack another entity, dealing damage to it.
///
/// The `Attack` action is implemented as a struct that implements the `Action` trait.
/// It takes a target entity as a parameter and returns a `ActionResult` indicating whether the attack was successful or not.
///
/// The `Attack` action can be used by any entity that has the ability to attack, such as a player or a monster.
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
