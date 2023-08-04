use std::fmt::Debug;

use crate::{map, world::World, Action, Tile};

pub struct ActionQueue<T: Tile> {
    actions: Vec<Box<dyn Action<T>>>,
}

impl<T: Tile> Debug for ActionQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionQueue")
            .field("actions", &self.actions.len())
            .finish()
    }
}

impl<T: Tile> ActionQueue<T> {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add(&mut self, action: Box<dyn Action<T>>) {
        self.actions.push(action);
    }

    pub fn add_all(&mut self, actions: Vec<Box<dyn Action<T>>>) {
        self.actions.extend(actions);
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn process_actions(&mut self, world: &mut World<T>, map: &mut map::Map<T>) {
        for action in self.actions.iter() {
            action.perform(world, map);
        }
        self.actions.clear();
    }
}
