#[derive(Debug, Clone)]
pub enum Visibility {
    Hidden(String),
    Visible(String),
}

#[derive(Debug, Clone)]
pub enum CellType {
    Floor,
    Wall,
    None,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub(crate) visibility: Visibility,
    // transparency: Transparency,
    pub(crate) cell_type: CellType,
}

#[derive(Debug, Clone)]
pub enum Transparency {
    Transparent,
    Opaque(String),
}

impl Tile {
    pub fn new(sprite_name: String) -> Self {
        Self {
            visibility: Visibility::Visible(sprite_name),
            ..Default::default()
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            // transparency: Transparency::Transparent,
            visibility: Visibility::Hidden("none".to_string()),
            cell_type: CellType::None,
        }
    }
}
