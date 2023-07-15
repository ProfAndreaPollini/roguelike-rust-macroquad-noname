#![allow(dead_code)]
// #[derive(Debug, Clone)]
// struct Effect {
//     name: String,
//     value: &'static dyn Fn(i32) -> i32,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loot {
    pub items: Vec<Item>,
    pub sprite_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gold {
    pub value: u16,
    pub sprite_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    // Vec<Effect>,
    Potion(String),
    Gold(Gold),
    Loot(Loot),
}

impl Item {
    pub fn sprite_name(&self) -> &str {
        match self {
            Item::Potion(p) => p.as_ref(),
            Item::Gold(g) => g.sprite_name.as_ref(),
            Item::Loot(l) => l.sprite_name.as_ref(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn remove_item(&mut self, item: &Item) {
        self.items.retain(|i| i != item);
    }

    pub fn contains(&self, item: &Item) -> bool {
        self.items.contains(item)
    }
}
