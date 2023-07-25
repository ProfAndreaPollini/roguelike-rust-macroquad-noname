use crate::engine::items::Item;

#[derive(Debug, Clone)]
pub enum Visibility {
    Hidden(String),
    Visible(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellType {
    Floor,
    Wall,
    None,
}

#[derive(Debug, Clone)]
pub struct Tile {
    visible: bool,
    // transparency: Transparency,
    pub(crate) cell_type: CellType,
    visible_sprite_name: String,
    explored_sprite_name: String,
    explored: bool,
    opaque: bool,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Transparency {
    Transparent,
    Opaque(String),
}

impl Tile {
    pub fn new(visible_sprite_name: String, explored_sprite_name: String) -> Self {
        Self {
            visible_sprite_name,
            explored_sprite_name,
            ..Default::default()
        }
    }

    pub fn sprite_name(&self) -> Option<&str> {
        if self.visible {
            Some(&self.visible_sprite_name)
        } else if self.explored {
            Some(&self.explored_sprite_name)
        } else {
            // Some(&self.visible_sprite_name)
            None
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn visible_sprite_name(&self) -> &str {
        self.visible_sprite_name.as_ref()
    }

    pub fn explored_sprite_name(&self) -> &str {
        self.explored_sprite_name.as_ref()
    }

    pub fn explored(&self) -> bool {
        self.explored
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_explored(&mut self, explored: bool) {
        self.explored = explored;
    }

    pub fn set_opaque(&mut self, opaque: bool) {
        self.opaque = opaque;
    }

    pub fn opaque(&self) -> bool {
        self.opaque
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            // transparency: Transparency::Transparent,
            visible: false,
            visible_sprite_name: String::from("none"),
            explored_sprite_name: String::from("none"),
            explored: false,
            cell_type: CellType::None,
            opaque: false,
            items: Vec::new(),
        }
    }
}
