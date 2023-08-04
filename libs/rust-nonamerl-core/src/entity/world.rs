#![allow(dead_code)]
use std::cell::RefCell;

use slotmap::new_key_type;

use crate::{entity::WithId, Tile};

use super::{
    entity::Entity,
    item::{Item, ItemBuilder},
};

new_key_type! { pub struct EntityKey; }
new_key_type! { pub struct ItemKey; }

#[derive(Debug)]
pub struct SlotMapStorage<K: slotmap::Key, V: WithId<K, V>> {
    pub data: slotmap::SlotMap<K, V>,
}

impl<K: slotmap::Key, V: WithId<K, V>> SlotMapStorage<K, V> {
    pub fn new() -> Self {
        Self {
            data: slotmap::SlotMap::with_key(),
        }
    }
    pub fn add<F: Fn(&mut V)>(&mut self, name: &str, setup_fn: F) -> K {
        let entity = V::create_with_id(Default::default(), name);
        let key = self.data.insert(entity);

        let new_data = self.data.get_mut(key).unwrap();
        setup_fn(new_data);
        println!("Added entity with key: {:?}", key);
        key
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.data.remove(key)
    }
}

#[derive(Debug)]
pub struct World<T: Tile> {
    pub entities: RefCell<SlotMapStorage<EntityKey, Entity>>,
    pub items: RefCell<SlotMapStorage<ItemKey, Item<T>>>,
}

impl<T: Tile> World<T> {
    pub fn new() -> Self {
        Self {
            entities: RefCell::new(SlotMapStorage::new()),
            items: RefCell::new(SlotMapStorage::new()),
        }
    }

    // pub fn add_entity<F: FnOnce(&mut Entity)>(&self, name: &str, setup_fn: F) -> EntityKey {
    //     // let entity = Entity::create_with_id(Default::default(), name);
    //     // let key = self.entities.insert(entity);

    //     // let entity = self.entities.get_mut(key).unwrap();
    //     // setup_fn(entity);
    //     // println!("Added entity with key: {:?}", key);
    //     // key
    //     self.entities.borrow_mut().add_entity(name, setup_fn)
    // }
    // pub fn get_entity(&self, key: EntityKey) -> Option<&Entity> {
    //     let e = self.entities.borrow().get_entity(key);
    //     e
    // }

    // pub fn get_entity_mut(&mut self, key: EntityKey) -> Option<&mut Entity> {
    //     self.entities.borrow_mut().get_entity_mut(key)
    // }

    // pub fn remove_entity(&mut self, key: EntityKey) -> Option<Entity> {
    //     self.entities.borrow_mut().remove(key)
    // }

    // pub fn add_item<F: FnOnce(&mut Item)>(&mut self, name: &str, setup_fn: F) -> ItemKey {
    //     self.items.borrow_mut().add_entity(name, setup_fn)
    // }

    // pub fn get_item(&self, key: ItemKey) -> Option<&Item> {
    //     self.items.borrow().get_entity(key)
    // }

    // pub fn get_item_mut(&mut self, key: ItemKey) -> Option<&mut Item> {
    //     self.items.borrow_mut().get_entity_mut(key)
    // }

    // pub fn remove_item(&mut self, key: ItemKey) -> Option<Item> {
    //     self.items.borrow_mut().remove(key)
    // }

    // pub fn create_item(&mut self, builder: ItemBuilder) -> &mut Item {
    //     let item = builder.build(self);
    //     item
    // }

    // pub fn get_entity(&self, key: EntityKey) -> Option<&Entity> {
    //     self.entities.get(key)
    // }

    // pub fn get_entity_mut(&mut self, key: EntityKey) -> Option<&mut Entity> {
    //     self.entities.get_mut(key)
    // }

    // pub fn remove_entity(&mut self, key: EntityKey) -> Option<Entity> {
    //     self.entities.remove(key)
    // }
}

impl<T: Tile> Default for World<T> {
    fn default() -> Self {
        Self::new()
    }
}

// #[cfg(test)]

// mod tests {
//     use crate::entity::{entity::Entity, property::HealthData, world::World};

//     #[test]
//     fn test_add_entity() {
//         let world = World::new();

//         let key = world.entities.borrow_mut().add("Test", |entity| {
//             entity.add_property(crate::entity::property::Property::Xp(6));
//         });

//         // let entity = world.get_entity_mut(entity_key).unwrap();
//         println!("Entity: {:?}", key);
//     }
// }
