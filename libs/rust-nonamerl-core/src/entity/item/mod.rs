#![allow(dead_code)]
use std::fmt::Display;

use crate::Tile;

use super::{
    activator::UseKind,
    property::HealthData,
    world::{ItemKey, World},
    WithId,
};

#[derive(Debug, Clone, Copy)]
pub struct Damage {
    pub value: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Defense {
    pub value: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Weapon {
    pub damage: Option<Damage>,
    pub defense: Option<Defense>,
}

#[derive(Debug, Clone, Copy)]
pub struct Food {}

#[derive(Debug, Clone, Copy)]
pub struct Potion {
    pub health: HealthData,
}

#[derive(Debug, Clone, Copy)]
pub enum ItemClass {
    Weapon(Weapon),
    Potion(Potion),
    Food(Food),
}

#[derive(Debug)]
pub struct Item<T: Tile> {
    id: ItemKey,
    pub name: String,
    pub class: ItemClass,
    pub activators: Vec<UseKind<T>>,
}

impl<T: Tile> Item<T> {
    pub fn new(id: ItemKey, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            class: ItemClass::Food(Food {}),
            activators: vec![],
        }
    }

    pub fn get_activator(&self, pos: usize) -> Option<&UseKind<T>> {
        self.activators.get(pos)
    }
}

impl<T: Tile> WithId<ItemKey, Item<T>> for Item<T> {
    fn id(&self) -> ItemKey {
        self.id
    }

    fn create_with_id(id: ItemKey, name: &str) -> Self {
        Self::new(id, name)
    }
}

impl<T: Tile> Display for Item<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("Item: {}", self.name);
        match &self.class {
            ItemClass::Weapon(weapon) => {
                s.push_str(&format!("\nDamage: {:?}", weapon.damage));
                s.push_str(&format!("\nDefense: {:?}", weapon.defense));
            }
            ItemClass::Food(_) => {}
            ItemClass::Potion(_) => {}
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct ItemBuilder<T: Tile> {
    name: String,
    class: ItemClass,
    activators: Vec<UseKind<T>>,
}

impl<T: Tile> ItemBuilder<T> {
    pub fn new(name: String, itemClass: ItemClass) -> Self {
        Self {
            name,
            class: itemClass,
            activators: vec![],
        }
    }

    pub fn add_activator(mut self, activator: UseKind<T>) -> Self {
        self.activators.push(activator);
        self
    }

    pub fn build<'a>(self, world: &'a World<T>) -> ItemKey {
        let k = world.items.borrow_mut().add(&self.name, move |item| {
            item.class = self.class;
            item.activators = self.activators.clone();
        });

        k
        // let item = world.items.borrow_mut().get_mut(k);
        // return item;
    }
}
