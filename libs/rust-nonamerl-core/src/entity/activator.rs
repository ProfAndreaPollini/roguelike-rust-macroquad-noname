#![allow(dead_code)]
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

pub type AttackActivateFn = fn(ItemKey, EntityKey, &World, EntityKey) -> Option<Box<dyn Action>>;
pub type DefenseActivateFn = fn(ItemKey, EntityKey, &World) -> Option<Box<dyn Action>>;
pub type PickActivateFn = fn(ItemKey, EntityKey, &World) -> Option<Box<dyn Action>>;
pub type DrinkActivateFn = fn(ItemKey, EntityKey, &World) -> Option<Box<dyn Action>>;

#[derive(Debug, Clone)]
pub enum UseKind {
    Attack { activate_fn: AttackActivateFn },
    Defense { activate_fn: DefenseActivateFn },
    Pick { activate_fn: PickActivateFn },
    Drink { activate_fn: DrinkActivateFn },
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
