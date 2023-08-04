#![allow(dead_code)]
use std::{cell::RefCell, fmt::Debug};

use crate::{
    dimension::{Dimension2, Dimension2D, IntExtent2D},
    grid::LatticeGrid2D,
    vector::{IntVector2, Vec2},
    SpriteSheet,
};

mod builder;
mod commands;
mod fov;
mod noise_builder;
mod random_walk_builder;
mod room;
mod room_builder;

pub use commands::*;
pub use fov::*;
pub use random_walk_builder::RandomWalkBuilder;
pub use room::*;
pub use room_builder::*;

pub use builder::{MapBuilder, MapBuilderAlgorithm};
use macroquad::{
    prelude::{Color, Rect},
    texture::Texture2D,
};
pub use noise_builder::BuilderAlgoWithNoise;

#[derive(Debug, Clone, PartialEq)]
pub enum TileSpriteInfo {
    SpriteSheet(&'static str),
    SingleSprite(Texture2D),
    Fill(Color),
    None,
}

pub trait Tile: 'static + Debug + Clone + Visible + Visited + FovOccluder {
    fn sprite_info(&self) -> TileSpriteInfo {
        TileSpriteInfo::None
    }
}

#[repr(transparent)]
#[derive(PartialEq)]
pub struct VisibilityOcclusion(f32);

impl VisibilityOcclusion {
    pub fn new(v: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&v) {
            Some(Self(v))
        } else {
            None
        }
    }

    pub unsafe fn new_unchecked(v: f32) -> Self {
        Self(v)
    }
}

impl From<VisibilityOcclusion> for f32 {
    fn from(v: VisibilityOcclusion) -> Self {
        v.0
    }
}

pub trait FovOccluder {
    const BLOCKED: VisibilityOcclusion = VisibilityOcclusion(0.);
    const VISIBLE: VisibilityOcclusion = VisibilityOcclusion(1.);
    fn block_visibility(&self) -> VisibilityOcclusion {
        Self::VISIBLE
    }
}

pub trait Visible {
    fn is_visible(&self) -> bool {
        true
    }
    fn set_visible(&mut self, visible: bool) {}
}

pub trait Visited {
    fn is_visited(&self) -> bool {
        false
    }
    fn set_visited(&mut self, visited: bool) {}
}

#[derive(Clone, Debug)]
pub struct Map<T: Tile> {
    grid: RefCell<LatticeGrid2D<T>>,
    extent: IntExtent2D,
    cell_size: Dimension2D<usize>,
    commands: RefCell<MapCommands>,
}

impl<T: Tile> Map<T> {
    pub fn new(extent: IntExtent2D, cell_size: Dimension2D<usize>) -> Self {
        Self {
            grid: RefCell::new(LatticeGrid2D::<T>::new()),
            extent,
            cell_size,
            commands: RefCell::new(MapCommands::default()),
        }
    }

    pub fn add_command(&self, command: MapCommand) {
        self.commands.borrow_mut().add(command);
    }

    pub fn add_commands(&self, commands: Vec<MapCommand>) {
        self.commands.borrow_mut().add_all(commands);
    }

    pub fn process_commands(&mut self) {
        for command in self.commands.borrow_mut().commands.iter() {
            match command {
                MapCommand::SetVisited(pos, visited) => {
                    self.set_visited(pos.x(), pos.y(), *visited);
                }
                MapCommand::SetVisible(pos, visible) => {
                    self.set_visible(pos.x(), pos.y(), *visible);
                }
            }
        }
        self.commands.borrow_mut().clear();
    }

    pub fn commands_available(&self) -> bool {
        !self.commands.borrow().is_empty()
    }

    pub fn set_visited(&self, x: i32, y: i32, visited: bool) {
        if let Some(tile) = self.grid.borrow_mut().at_mut(IntVector2::new(x, y)) {
            tile.set_visited(visited);
        }
    }

    pub fn set_visible(&self, x: i32, y: i32, visible: bool) {
        if let Some(tile) = self.grid.borrow_mut().at_mut(IntVector2::new(x, y)) {
            tile.set_visible(visible);
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<T> {
        let binding = self.grid.borrow();
        match self.grid.borrow().at(IntVector2::new(x, y)) {
            Some(tile) => Some((*tile).clone()),
            None => None,
        }
    }

    pub fn set(&self, x: i32, y: i32, tile: T) {
        self.grid.borrow_mut().put(IntVector2::new(x, y), tile);
    }

    pub fn size(&self) -> IntExtent2D {
        self.extent
    }

    pub fn len(&self) -> usize {
        self.grid.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.grid.borrow().is_empty()
    }

    pub fn cell_size(&self) -> Dimension2D<usize> {
        self.cell_size
    }

    pub fn line(&self, start: IntVector2, end: IntVector2) -> Vec<IntVector2> {
        self.grid.borrow().line(start, end)
    }

    pub fn coords_of_cell(&self, x: i32, y: i32) -> Option<IntVector2> {
        if self.extent.contains(x, y) {
            Some(IntVector2::new(
                x * self.cell_size.width() as i32,
                y * self.cell_size.height() as i32,
            ))
        } else {
            None
        }
    }

    // pub fn fov_iter(&self, center: IntVector2, radius: i32) -> MapFovIter<T> {
    //     MapFovIter::new(self, center, radius)
    // }
}

#[cfg(test)]

mod tests {
    use crate::vector::IntVector2;

    use super::*;

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct TestTile {}
    impl Tile for TestTile {}
    impl Visible for TestTile {}
    impl Visited for TestTile {}
    impl FovOccluder for TestTile {}

    #[test]
    fn test_map() {
        let mut map =
            Map::<TestTile>::new(IntExtent2D::new(0, 0, 10, 10), Dimension2D::new(24, 24));
        map.grid
            .borrow_mut()
            .put(IntVector2::new(1, 1), TestTile::default());
        map.grid
            .borrow_mut()
            .put(IntVector2::new(0, 1), TestTile::default());
        assert_eq!(map.get(1, 1), Some(TestTile::default()));
        assert_eq!(map.get(0, 1), Some(TestTile::default()));
        assert_eq!(map.get(0, 0), None);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_cell_coords() {
        let map = Map::<TestTile>::new(IntExtent2D::new(0, 0, 10, 10), Dimension2D::new(24, 24));
        assert_eq!(map.coords_of_cell(0, 0), Some(IntVector2::new(0, 0)));
        assert_eq!(map.coords_of_cell(1, 1), Some(IntVector2::new(24, 24)));
        assert_eq!(map.coords_of_cell(0, 1), Some(IntVector2::new(0, 24)));
        assert_eq!(map.coords_of_cell(1, 0), Some(IntVector2::new(24, 0)));
        assert_eq!(map.coords_of_cell(10, 10), None);
    }
}
