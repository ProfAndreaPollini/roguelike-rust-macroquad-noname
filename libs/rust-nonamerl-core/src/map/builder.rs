use std::collections::HashMap;

use crate::{Dimension2D, IntExtent2D, IntVector2, Map, Room, Tile};

/// A trait for defining algorithms that can be used to build maps.
pub trait MapBuilderAlgorithm<T: Tile> {
    /// Builds a map using the given `MapBuilder`.
    ///
    /// # Arguments
    ///
    /// * `map_builder` - A mutable reference to a `MapBuilder` instance.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `MapBuilder` instance.
    fn build<'a>(&self, map_builder: &'a mut MapBuilder<T>) -> &'a mut MapBuilder<T>;
}

#[derive(Clone, Debug)]
pub struct MapBuilder<T: Tile> {
    /// The extent of the map
    pub map: Map<T>,
    pub rooms: Vec<Room>,
    /// The different types of tiles that can be used to build the map.
    pub(super) tiles: HashMap<String, T>,
}

impl<T: Tile> MapBuilder<T> {
    pub fn new(extent: IntExtent2D, cell_size: Dimension2D<usize>) -> Self {
        Self {
            map: Map::new(extent, cell_size),
            tiles: HashMap::new(),
            rooms: Vec::new(),
        }
    }

    pub fn add_tile(&mut self, name: &str, tile: T) {
        self.tiles.insert(name.to_string(), tile);
    }

    /// Adds a step to the map building process using the given `MapBuilderAlgorithm`.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - A reference to an implementation of the `MapBuilderAlgorithm` trait.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `MapBuilder` instance.
    pub fn add_step<'a>(&'a mut self, algorithm: &'a impl MapBuilderAlgorithm<T>) -> &'a mut Self {
        let builder = algorithm.build(self);
        builder
    }

    // Builds a `Map` instance using the current state of the `MapBuilder`.
    // pub fn build(self) -> Map<T> {
    //     self.map
    // }
}

#[derive(Debug, Copy, Clone)]
struct FillWithFloorBuilderAlgo<T>
where
    T: Clone,
{
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone> FillWithFloorBuilderAlgo<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Tile> MapBuilderAlgorithm<T> for FillWithFloorBuilderAlgo<T> {
    fn build<'a>(&self, map_builder: &'a mut MapBuilder<T>) -> &'a mut MapBuilder<T> {
        for x in map_builder.map.extent.left()..map_builder.map.extent.right() {
            for y in map_builder.map.extent.top()..map_builder.map.extent.bottom() {
                map_builder
                    .map
                    .set(x, y, map_builder.tiles["grass"].clone());
            }
        }

        map_builder
    }
}

#[cfg(test)]

mod tests {

    use crate::{FovOccluder, Visible, Visited, Walkable};

    use super::*;

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct TestTile {}
    impl Tile for TestTile {}
    impl Visible for TestTile {}
    impl Visited for TestTile {}
    impl FovOccluder for TestTile {}
    impl Walkable for TestTile {}

    #[test]
    fn test_map_builder() {
        let mut map_builder =
            MapBuilder::<TestTile>::new(IntExtent2D::new(0, 0, 10, 10), Dimension2D::new(24, 24));

        map_builder.add_tile("grass", TestTile {});
        map_builder.add_tile("water", TestTile {});

        assert_eq!(map_builder.tiles.len(), 2);
    }

    #[test]
    fn test_map_builder_add_step() {
        let mut map_builder =
            MapBuilder::<TestTile>::new(IntExtent2D::new(0, 0, 10, 10), Dimension2D::new(24, 24));
        map_builder.add_tile("grass", TestTile {});
        map_builder.add_tile("water", TestTile {});
        map_builder.add_step(&FillWithFloorBuilderAlgo::<TestTile>::new());

        assert_eq!(map_builder.map.len(), 100);

        // assert_eq!(map_builder.map.grid, 100);
    }
}
