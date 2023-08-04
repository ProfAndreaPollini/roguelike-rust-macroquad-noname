use std::fmt::Debug;

use crate::{map, world::World, Action, Tile};

/// A queue of actions that can be performed on the world and map.
pub struct ActionQueue<T: Tile> {
    actions: Vec<Box<dyn Action<T>>>,
}

/// A debug implementation that only shows the number of actions in the queue.
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

    /// Adds an action to the queue.
    pub fn add(&mut self, action: Box<dyn Action<T>>) {
        self.actions.push(action);
    }

    /// Adds all actions from the given vector to the queue.
    pub fn add_all(&mut self, actions: Vec<Box<dyn Action<T>>>) {
        self.actions.extend(actions);
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Clears the action queue.
    pub fn clear(&mut self) {
        self.actions.clear();
    }

    /// Process all actions in the queue, updating the world and map accordingly.
    pub fn process_actions(&mut self, world: &mut World<T>, map: &mut map::Map<T>) {
        for action in self.actions.iter() {
            action.perform(world, map);
        }
        self.actions.clear();
    }
}

impl<T: Tile> Default for ActionQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
