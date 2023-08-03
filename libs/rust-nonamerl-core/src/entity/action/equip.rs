#![allow(dead_code)]
use crate::entity::{
    property::Property,
    world::{EntityKey, ItemKey, World},
};

use super::Action;

#[derive(Debug, Clone, Copy)]
pub struct EquipAction {
    pub item: ItemKey,
    pub target: EntityKey,
}

impl EquipAction {
    pub fn new(item: ItemKey, target: EntityKey) -> Self {
        Self { item, target }
    }
}

impl Action for EquipAction {
    fn perform(&self, world: &World) {
        let mut entities = world.entities.borrow_mut();
        let target = entities.get_mut(self.target).unwrap();
        let items = world.items.borrow();
        let item = items.get(self.item).unwrap();
        target.add_property(Property::Equip(self.item));
        println!("Equip action to target {:?} with item {:?}", target, item);
    }
}
