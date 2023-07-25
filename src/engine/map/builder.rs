use std::collections::HashMap;

use noise::{NoiseFn, Perlin, Seedable};

use crate::engine::level::Room;

use super::{
    tile::{CellType, Tile},
    Map, MapTiles,
};

#[derive(Debug)]
/// A builder for creating maps.
pub struct MapBuilder {
    /// The width of the map.
    pub width: u32,
    /// The height of the map.
    pub height: u32,
    /// The tiles that make up the map.
    pub map_tiles: MapTiles,
    pub rooms: Vec<Room>,
    /// The different types of tiles that can be used to build the map.
    tiles: HashMap<String, Tile>,
}

/// A trait for defining algorithms that can be used to build maps.
pub trait MapBuilderAlgorithm<'a> {
    /// Builds a map using the given `MapBuilder`.
    ///
    /// # Arguments
    ///
    /// * `map_builder` - A mutable reference to a `MapBuilder` instance.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `MapBuilder` instance.
    fn build(&self, map_builder: &'a mut MapBuilder) -> &'a mut MapBuilder;
}

#[derive(Debug, Default)]
/// A basic map builder algorithm that generates a map using Perlin noise.
pub struct BasicMapBuilder {}

/// An implementation of the `MapBuilderAlgorithm` trait that generates a basic map using Perlin noise.
impl<'a> MapBuilderAlgorithm<'a> for BasicMapBuilder {
    fn build(&self, map_builder: &'a mut MapBuilder) -> &'a mut MapBuilder {
        let perlin = Perlin::new(1);
        let mut seed = 0;
        let mut noise = |x, y| {
            seed += 1;
            perlin.set_seed(seed);
            perlin.get([x, y])
        };

        for x in 0..map_builder.width {
            for y in 0..map_builder.height {
                let tile = if noise(x as f64 / 30., y as f64 / 30.).abs() > 0.2 {
                    let mut t = Tile::new("test".to_string(), "test".to_string());
                    t.cell_type = CellType::Floor;
                    t
                    // tile.add_sprite("floor", 2, 2);
                } else {
                    let mut t = Tile::new("wall".to_string(), "wall".to_string());
                    t.cell_type = CellType::Wall;
                    t.set_opaque(true);
                    t
                };

                map_builder.map_tiles.add_tile(x as u16, y as u16, tile);
            }
        }
        map_builder
    }
}

/// A builder for creating maps.
impl MapBuilder {
    pub fn new(width: u32, height: u32, tiles: HashMap<String, Tile>) -> Self {
        Self {
            width,
            height,
            map_tiles: MapTiles::new(),
            tiles,
            rooms: vec![],
        }
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
    pub fn add_step<'a>(&'a mut self, algorithm: &'a impl MapBuilderAlgorithm<'a>) -> &'a mut Self {
        let builder = algorithm.build(self);
        builder
    }

    /// Builds a `Map` instance using the current state of the `MapBuilder`.
    pub fn build(&self) -> Map {
        let mut map = Map::new(self.width, self.height);
        map.tiles = self.map_tiles.clone();
        map.rooms = self.rooms.clone();
        map
    }
}
