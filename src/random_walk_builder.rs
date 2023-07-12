use crate::engine::map::builder::MapBuilderAlgorithm;
use macroquad::prelude::Vec2;
use rand::prelude::*;
#[derive(Debug, Clone, Default)]
pub struct RandomWalkBuilder {}

impl<'a> MapBuilderAlgorithm<'a> for RandomWalkBuilder {
    fn build(
        &self,
        map_builder: &'a mut crate::engine::map::builder::MapBuilder,
    ) -> &'a mut crate::engine::map::builder::MapBuilder {
        let mut rng = thread_rng();
        let pos = Vec2::new(10., 10.);

        let mut current_pos = pos;

        let mut visited = vec![pos];
        let directions = ["up", "down", "left", "right"];
        // generate a random walk
        for _ in 0..100 {
            let mut next_pos = current_pos;

            // randomly choose a direction
            let direction = directions.choose(&mut rng).unwrap();
            //let direction = directions[dir];

            match *direction {
                "up" => next_pos.y -= 1.,
                "down" => next_pos.y += 1.,
                "left" => next_pos.x -= 1.,
                "right" => next_pos.x += 1.,
                _ => {}
            }

            visited.push(next_pos);

            current_pos = next_pos;
        }
        println!("visited: {:?}", visited);
        //map_builder.map_tiles.tiles = visited.clone();
        visited.iter().for_each(|pos| {
            let mut tile = crate::engine::map::tile::Tile::new("test".to_string());
            tile.cell_type = crate::engine::map::tile::CellType::Floor;
            map_builder
                .map_tiles
                .add_tile(pos.x as u16, pos.y as u16, tile);
        });
        println!("map_tiles: {:?}", map_builder.map_tiles.tiles);

        map_builder
    }
}
