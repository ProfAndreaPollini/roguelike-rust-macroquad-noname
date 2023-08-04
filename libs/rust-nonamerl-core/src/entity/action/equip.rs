#![allow(dead_code)]
use crate::{
    entity::{
        property::Property,
        world::{EntityKey, ItemKey, World},
    },
    Map, Tile,
};

use super::Action;

#[derive(Debug, Clone, Copy)]
pub struct EquipAction<T: Tile> {
    pub item: ItemKey,
    pub target: EntityKey,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Tile> EquipAction<T> {
    pub fn new(item: ItemKey, target: EntityKey) -> Self {
        Self {
            item,
            target,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Tile> Action<T> for EquipAction<T> {
    fn perform(&self, world: &World<T>, _map: &mut Map<T>) {
        let mut entities = world.entities.borrow_mut();
        let target = entities.get_mut(self.target).unwrap();
        let items = world.items.borrow();
        let item = items.get(self.item).unwrap();
        target.add_property(Property::Equip(self.item));
        println!("Equip action to target {:?} with item {:?}", target, item);
    }
}
