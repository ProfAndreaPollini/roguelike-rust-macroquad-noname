#![allow(dead_code)]
use std::collections::HashMap;

use super::{
    property::Property,
    world::{EntityKey, World},
    WithId,
};

// pub struct EntityBuilder {
//     name: String,
//     health: i32,
//     xp: i32,
//     properties: HashMap<&'static str, Property>,
// }

#[derive(Debug)]
pub struct Entity {
    id: EntityKey,
    name: String,
    health: i32,
    // xp: i32,
    // inventory: Option<Inventory>,
    properties: HashMap<&'static str, Property>,
}

impl Entity {
    fn new(id: EntityKey, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            health: 100,
            properties: HashMap::new(),
        }
    }

    pub fn has_property(&self, property_name: &str) -> bool {
        self.properties.contains_key(property_name)
    }

    pub fn get_property(&self, property_name: &str) -> Option<&Property> {
        self.properties.get(property_name)
    }

    pub fn get_property_mut(&mut self, property_name: &str) -> Option<&mut Property> {
        self.properties.get_mut(property_name)
    }

    pub fn add_property(&mut self, property: Property) {
        self.properties.insert(property.name(), property);
    }
}

impl WithId<EntityKey, Entity> for Entity {
    fn id(&self) -> EntityKey {
        self.id
    }

    fn create_with_id(id: EntityKey, name: &str) -> Self {
        Self::new(id, name)
    }
}

// #[derive(Debug)]
// pub struct EntityBuilder {
//     name: String,
//     properties: HashMap<&'static str, Property>,
// }

// impl EntityBuilder {
//     pub fn new(name: String) -> Self {
//         Self {
//             name,
//             properties: HashMap::new(),
//         }
//     }

//     pub fn add_property(&mut self, property: Property) -> &Self {
//         self.properties.insert(property.name(), property);
//         self
//     }

//     pub fn build(self, world: &World) -> &mut Entity {
//         let k = world.entities.borrow_mut().add(&self.name, move |entity| {
//             entity.properties = self.properties;
//         });

//         let out = world.entities.borrow_mut().get_mut(k);
//         out.unwrap()
//     }
// }

#[cfg(test)]
mod tests {

    use std::{cell::RefMut, rc::Rc};

    use crate::entity::{
        action::{Action, AttackAction, EquipAction},
        activator::{AttackActivateFn, UseKind},
        item::{Damage, Item, ItemBuilder, ItemClass, Potion, Weapon},
        property::HealthData,
        world::{ItemKey, World},
    };

    use super::*;

    #[test]
    fn test_sword() {
        let world = World::new();

        let sword_key = {
            let builder = ItemBuilder::new(
                "Sword".to_string(),
                ItemClass::Weapon(Weapon {
                    damage: Some(Damage { value: 10 }),
                    defense: None,
                }),
            )
            .add_activator(UseKind::Attack {
                activate_fn: |item_key: ItemKey,
                              entity: EntityKey,
                              world: &World,
                              target: EntityKey| {
                    let entities = world.entities.borrow();
                    let entity = entities.get(entity).unwrap();
                    let items = world.items.borrow();
                    let item = items.get(item_key).unwrap();
                    if entity.health > 50 {
                        /* customizable */
                        let damage = match &item.class {
                            ItemClass::Weapon(weapon) => weapon.damage.unwrap().value,
                            _ => 0,
                        };
                        Some(Box::new(AttackAction::new(damage).to(target)))
                    } else {
                        None
                    }
                },
            });
            builder.build(&world)
        };

        let mut player_key = world.entities.borrow_mut().add("Player", |player| {
            player.properties.insert(Property::XP, Property::Xp(5));
        });

        let mut entities = world.entities.borrow_mut();
        let player = entities.get_mut(player_key).unwrap();
        let mut items = world.items.borrow_mut();
        let sword = items.get_mut(sword_key).unwrap();

        //     // let player_key = world.add_entity("Player", |player| {
        //     //     player.properties.insert(Property::XP, Property::Xp(5));
        //     // });

        //     // let player = world.get_entity_mut(player_key).unwrap();

        let check_is_weapon = |item: &Item| matches!(&item.class, ItemClass::Weapon(_));

        let can_attack = |item: &Item, entity: &Entity| match &item.class {
            ItemClass::Weapon(_) => entity.health > 50,
            _ => false,
        };

        let decrease_health = |entity: &mut Entity, health: i32| {
            entity.health -= health;
        };

        //     // assert!(check_is_weapon(&sword));
        assert!(can_attack(&sword, &player));
        //     // assert_eq!(
        //     //     sword.activators.first().unwrap().can_use(&player),
        //     //     UseResult::Ok(Box::new(DummyAction {})
        //     // );

        decrease_health(player, 70);
        assert!(!can_attack(&sword, &player));

        //     // assert_eq!(
        //     //     sword.activators.first().unwrap().can_use(&player),
        //     //     UseResult::Failed
        //     // );

        assert!(player.has_property("xp"));

        let xp = player.get_property("xp").unwrap();
        assert!(matches!(xp, Property::Xp(_)));
        assert_eq!(xp.name(), Property::XP);
    }

    #[test]
    fn test_entity() {
        let mut world = World::new();
        let mut player_key = world.entities.borrow_mut().add("Player", |player| {
            player.properties.insert(Property::XP, Property::Xp(5));
        });

        let mut entities = world.entities.borrow_mut();
        let player = entities.get_mut(player_key).unwrap();

        assert!(player.has_property(Property::XP));

        let xp = player.get_property("xp").unwrap();
        assert!(matches!(xp, Property::Xp(_)));
        assert_eq!(xp.name(), Property::XP);
        player.add_property(Property::Health(HealthData { health: 100 }));
    }

    #[test]
    fn test_weapon_equip() {
        let mut world = World::new();
        let player_key = {
            let mut entities = world.entities.borrow_mut();

            let mut player_key = entities.add("Player", |player| {
                player.add_property(Property::Xp(5));
                player.add_property(Property::Health(HealthData { health: 100 }));
            });
            // entities.get(player_key).unwrap()
            player_key
        };
        let sword_key = {
            let builder = ItemBuilder::new(
                "Sword".to_string(),
                ItemClass::Weapon(Weapon {
                    damage: Some(Damage { value: 10 }),
                    defense: None,
                }),
            )
            .add_activator(UseKind::Attack {
                activate_fn: |item_key: ItemKey,
                              key: EntityKey,
                              world: &World,
                              target_key: EntityKey| {
                    let entities = world.entities.borrow();
                    let entity = entities.get(key).unwrap();
                    let items = world.items.borrow();
                    let item = items.get(item_key).unwrap();

                    if entity.health > 50 {
                        /* customizable */
                        let damage = match &item.class {
                            ItemClass::Weapon(weapon) => weapon.damage.unwrap().value,
                            _ => 0,
                        };
                        Some(Box::new(AttackAction::new(damage).to(target_key)))
                    } else {
                        None
                    }
                },
            });
            builder.build(&world)
        };
        let mut items = world.items.borrow();
        let sword = { items.get(sword_key).unwrap() };

        let use_kind = sword.get_activator(0).unwrap();
        match use_kind {
            UseKind::Attack { activate_fn } => {
                let action = activate_fn(sword_key, player_key, &world, player_key);
                if let Some(action) = action {
                    action.perform(&world);
                }
            }
            _ => {}
        }

        // let mut entities = world.entities.borrow_mut();
        // let player = entities.get_mut(player_key).unwrap();

        // player.add_property(Property::Equip(sword_key));

        // let equip = player.get_property("equip").unwrap();
        // assert!(matches!(equip, Property::Equip(_)));
        // assert_eq!(equip.name(), Property::EQUIP);

        // let sword_key = match equip {
        //     Property::Equip(item) => {
        //         let items = world.items.borrow();
        //         let item = items.get(*item).unwrap();
        //         assert!(matches!(item.class, ItemClass::Weapon(_)));
        //         item.id()
        //     }
        //     _ => panic!("Not a weapon"),
        // };

        let equip = EquipAction::new(sword_key, player_key);

        equip.perform(&world);

        let entities = world.entities.borrow();
        let player = entities.get(player_key).unwrap();
        println!("Player: {:?}", player);
        // assert!(sword.activators.first().unwrap().can_use(&player) == UseResult::Ok);

        // let mut player2_key = {
        //     let mut entities = world.entities.borrow_mut();

        //     entities.add("Player2", |player| {
        //         player.add_property(Property::Xp(5));
        //         player.add_property(Property::Health(HealthData { health: 100 }));
        //     })
        // };
        // let player2 = entities.get_mut(player2_key).unwrap();
        // let sword = world.items.borrow().get(sword_key).unwrap();
        // let action = sword.activators.first().unwrap();

        // let action = match action {
        //     UseKind::Attack { activate } => {
        //         let action = (activate)(&player, player2_key).unwrap();
        //         action
        //     }
        //     _ => panic!("Not an attack"),
        // };
    }

    // #[test]
    // fn test_pick_item() {
    //     let mut world = World::new();
    //     let player_key = {
    //         world.add_entity("Player", |player| {
    //             player.add_property(Property::Xp(5));
    //             player.add_property(Property::Health(HealthData { health: 100 }));
    //         })
    //     };

    //     let player = { world.get_entity_mut(player_key).unwrap() };
    //     let potion_key = {
    //         world.add_item("Potion", |item| {
    //             item.class = ItemClass::Potion(Potion {
    //                 health: HealthData { health: 10 },
    //             });
    //             item.activators = vec![
    //                 UseKind::Attack {
    //                     activate: |_entity: &Entity, _target: &Entity| {
    //                         Some(Box::new(DummyAction {}))
    //                     },
    //                 },
    //                 UseKind::Pick {
    //                     activate: |_entity: &Entity| Some(Box::new(DummyAction {})),
    //                 },
    //                 UseKind::Drink {
    //                     activate: |_entity: &Entity| Some(Box::new(DummyAction {})),
    //                 },
    //             ];
    //         })
    //     };

    //     let potion = world.get_item_mut(potion_key).unwrap();

    //     match potion.activators.first().unwrap() {
    //         UseKind::Pick { activate } => {
    //             let action = (activate)(&player).unwrap();
    //             action.perform();
    //         }
    //         UseKind::Attack { activate } => {
    //             let action = (activate)(&player, &player).unwrap();
    //             action.perform();
    //         }
    //         _ => {}
    //     }

    //     // if let Some(UseKind::Pick { activate }) = potion.activators.first() {
    //     //     let action = (activate)(&player).unwrap();
    //     //     action.perform();
    //     // }

    //     // if let Some(UseKind::Drink { activate }) = potion.activators.last() {
    //     //     let action = (activate)(&player).unwrap();
    //     //     action.perform();
    //     // }

    //     // if let Some(UseKind::Attack { activate }) = potion.activators.get(0) {
    //     //     let action = (activate)(&player, &player).unwrap();
    //     //     action.perform();
    //     // }

    //     //let action = activate_item(&mut player, &potion, UseKind::Pick);
    //     // potion.activators[1].use(&player);
    //     // action.perform();
    //     // let action = sword.activators.first().unwrap().perform(&player2, &player);
    // }
}
