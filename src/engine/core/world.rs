use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::{
    borrow::BorrowMut,
    sync::{Mutex, MutexGuard, OnceLock},
};

use slotmap::{new_key_type, SlotMap};

use crate::engine::map::Map;

use super::entity::Entity;

new_key_type! { pub struct EntityKey; }

pub struct World {
    entities: SlotMap<EntityKey, Entity>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
        }
    }

    pub fn create_entity(&mut self, entity: Entity) -> (EntityKey, &Entity) {
        let key = {
            let ref mut this = self.entities;
            this.insert_with_key(|_| entity)
        };
        let e = self.entities.get_mut(key).unwrap();
        e.set_id(key);
        (key, e)
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityKey {
        self.entities.insert(entity)
    }

    pub fn get_entity(&self, key: EntityKey) -> Option<&Entity> {
        self.entities.get(key)
    }

    pub fn get_entity_mut(&mut self, key: EntityKey) -> Option<&mut Entity> {
        self.entities.get_mut(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut()
    }

    pub fn keys(&self) -> Vec<EntityKey> {
        self.entities.keys().collect::<Vec<EntityKey>>()
    }
}

// tests

#[cfg(test)]

mod tests {
    use crate::engine::core::entity::EntityFeatures;

    use super::*;

    #[test]
    fn test_world() {
        let mut world = World::new();

        let entity = Entity::Player(EntityFeatures::new("Player".to_string()));
        let entity_key = world.add_entity(entity);

        let entity = world.get_entity(entity_key).unwrap();

        match entity {
            Entity::Player(_) => {}
            _ => panic!("Expected player entity"),
        }
    }
}
