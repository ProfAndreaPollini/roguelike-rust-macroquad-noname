use std::collections::{HashMap, HashSet};

use rand::seq::{IteratorRandom, SliceRandom};

use crate::{IntVector2, MapBuilder, MapBuilderAlgorithm, Tile, Vec2};

#[derive(Debug, Clone)]
pub struct RandomWalkBuilder<T>
where
    T: Tile,
{
    start_pos: IntVector2,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Tile> RandomWalkBuilder<T> {
    pub fn new(start_pos: IntVector2) -> Self {
        Self {
            start_pos,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Tile> MapBuilderAlgorithm<T> for RandomWalkBuilder<T> {
    fn build<'a>(&self, map_builder: &'a mut MapBuilder<T>) -> &'a mut MapBuilder<T> {
        let mut rng = rand::thread_rng();
        let pos = self.start_pos;

        let mut current_pos = self.start_pos;

        let mut visited = HashSet::<IntVector2>::new();
        let directions = ["up", "down", "left", "right"];
        // generate a random walk
        while visited.len() < 100 {
            let mut next_pos = current_pos;

            // randomly choose a direction
            let direction = directions.choose(&mut rng).unwrap();
            //let direction = directions[dir];

            match *direction {
                "up" => *next_pos.x_mut() -= 1,
                "down" => *next_pos.y_mut() += 1,
                "left" => *next_pos.x_mut() -= 1,
                "right" => *next_pos.x_mut() += 1,
                _ => {}
            }

            if !visited.insert(next_pos) {
                // select random element from visited
                current_pos = *visited.iter().choose(&mut rng).unwrap();
            } else {
                current_pos = next_pos;
            }
        }
        println!("visited: {:?}", visited);
        //map_builder.map_tiles.tiles = visited.clone();
        visited.iter().for_each(|pos| {
            let tile = map_builder.tiles.get("floor").unwrap().clone();

            map_builder.map.set(pos.x(), pos.y(), tile);
        });
        // println!("map_tiles: {:?}", map_builder.map_tiles.tiles);

        map_builder
    }
}
