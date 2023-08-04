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
mod tile;

pub use builder::{MapBuilder, MapBuilderAlgorithm};
pub use commands::*;
pub use fov::*;
use macroquad::{
    prelude::{Color, Rect},
    texture::Texture2D,
};
pub use noise_builder::BuilderAlgoWithNoise;
pub use random_walk_builder::RandomWalkBuilder;
pub use room::*;
pub use room_builder::*;
pub use tile::*;

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
                MapCommand::AddItem(pos, item) => {
                    self.add_item(pos.x(), pos.y(), *item);
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

    fn add_item(&self, x: i32, y: i32, item: crate::world::ItemKey) {
        if let Some(tile) = self.grid.borrow_mut().at_mut(IntVector2::new(x, y)) {
            tile.add_item(item);
        }
    }

    pub fn iter_over_visible_tiles<'a>(
        &'a self,
        extent: &'a IntExtent2D,
    ) -> MapVisibleTilesIter<T> {
        MapVisibleTilesIter::new(self, extent)
    }

    // pub fn fov_iter(&self, center: IntVector2, radius: i32) -> MapFovIter<T> {
    //     MapFovIter::new(self, center, radius)
    // }
}

pub struct MapVisibleTilesIter<'a, T: Tile> {
    map: &'a Map<T>,
    extent: &'a IntExtent2D,
    current: IntVector2,
}

impl<'a, T: Tile> MapVisibleTilesIter<'a, T> {
    pub fn new(map: &'a Map<T>, extent: &'a IntExtent2D) -> Self {
        Self {
            map,
            extent,
            current: IntVector2::new(extent.left(), extent.top()),
        }
    }
}

impl<'a, T: Tile> Iterator for MapVisibleTilesIter<'a, T> {
    type Item = (IntVector2, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y() >= self.extent.bottom() {
            return None;
        }

        let result = self.current;

        self.current = IntVector2::new(self.current.x() + 1, self.current.y());

        if self.current.x() >= self.extent.right() {
            self.current = IntVector2::new(self.extent.left(), self.current.y() + 1);
        }

        if let Some(tile) = self.map.get(result.x(), result.y()) {
            // if tile.is_visible() {
            return Some((result, tile));
            // }
        }

        self.next()
    }
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
    impl Walkable for TestTile {}
    impl ItemContainer for TestTile {}

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
