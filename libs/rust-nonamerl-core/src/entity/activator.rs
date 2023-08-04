#![allow(dead_code)]
use crate::Tile;

use super::{
    action::Action,
    world::{EntityKey, ItemKey, World},
};

// pub enum UseResult {
//     Ok(Box<dyn Action>),
//     Failed,
// }

// impl PartialEq for UseResult {
//     fn eq(&self, other: &Self) -> bool {
//         matches!(
//             (self, other),
//             (UseResult::Ok(_), UseResult::Ok(_)) | (UseResult::Failed, UseResult::Failed)
//         )
//     }
// }

// impl std::fmt::Debug for UseResult {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             UseResult::Ok(action) => write!(f, "Ok()"),
//             UseResult::Failed => write!(f, "Failed"),
//         }
//     }
// }

pub type AttackActivateFn<T: Tile> =
    fn(ItemKey, EntityKey, &World<T>, EntityKey) -> Option<Box<dyn Action<T>>>;
pub type DefenseActivateFn<T: Tile> =
    fn(ItemKey, EntityKey, &World<T>) -> Option<Box<dyn Action<T>>>;
pub type PickActivateFn<T: Tile> = fn(ItemKey, EntityKey, &World<T>) -> Option<Box<dyn Action<T>>>;
pub type DrinkActivateFn<T: Tile> = fn(ItemKey, EntityKey, &World<T>) -> Option<Box<dyn Action<T>>>;

#[derive(Debug, Clone)]
pub enum UseKind<T: Tile> {
    Attack { activate_fn: AttackActivateFn<T> },
    Defense { activate_fn: DefenseActivateFn<T> },
    Pick { activate_fn: PickActivateFn<T> },
    Drink { activate_fn: DrinkActivateFn<T> },
}

// impl UseKind {
//     pub fn can_activate(
//         &self,
//         item: ItemKey,
//         key: EntityKey,
//         world: &World,
//     ) -> Option<Box<dyn Action>> {
//         match self {
//             UseKind::Pick {
//                 activate_fn: activate,
//             } => (activate)(item, key, world),
//             UseKind::Attack {
//                 activate_fn: activate,
//             } => (activate)(item, key, world),
//             UseKind::Defense {
//                 activate_fn: activate,
//             } => (activate)(item, key, world),
//             UseKind::Drink {
//                 activate_fn: activate,
//             } => (activate)(item, key, world),
//             _ => None,
//         }
//     }
// }

// #[derive(Debug)]

// pub struct Activator {
//     pub kind: UseKind,
//     pub activate_if: fn(&Entity) -> UseResult, // Box<dyn Action>
//                                                // perform: fn(&Entity, &Entity) -> UseResult,
// }

// impl Activator {
//     pub fn can_use(&self, entity: &Entity) -> UseResult {
//         (self.activate_if)(entity)
//     }
// }
