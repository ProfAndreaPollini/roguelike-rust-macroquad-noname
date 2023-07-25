use rand::thread_rng;

use crate::engine::{
    items::{Gold, Item},
    level::{Dimension, Room},
    map::{self, builder::MapBuilderAlgorithm, cell::Cell},
};

#[derive(Debug, Default)]
pub struct RoomBuilder {}

impl<'a> MapBuilderAlgorithm<'a> for RoomBuilder {
    fn build(
        &self,
        map_builder: &'a mut crate::engine::map::builder::MapBuilder,
    ) -> &'a mut crate::engine::map::builder::MapBuilder {
        let mut rng = thread_rng();

        let mut rooms = Vec::<Room>::new();

        let mut room = Room::create_random(20, 20);
        rooms.push(room);

        room =
            Room::create_random_in_rect(Cell::new(12, 12), Dimension::new(20, 20), (5..15, 5..15));
        rooms.push(room);

        let mut attempts = 0;
        while rooms.len() < 10 && attempts < 1000 {
            let candidate = Room::create_random_in_rect(
                Cell::new(3, 3),
                Dimension::new(80, 80),
                (10..25, 10..25),
            );
            for room in rooms.iter() {
                if candidate.intersects(room) {
                    attempts += 1;
                    continue;
                }
            }
            rooms.push(candidate);
        }

        for room in rooms.iter() {
            for cell in room.interior_cells() {
                let mut tile =
                    crate::engine::map::tile::Tile::new("floor".to_string(), "floor".to_string());
                tile.cell_type = crate::engine::map::tile::CellType::Floor;
                tile.set_visible(false);
                // tile.add_item(Item::Gold(Gold {
                //     value: 1,
                //     sprite_name: "gold".to_string(),
                // }));
                map_builder.map_tiles.add_tile(cell.x, cell.y, tile);
            }
            for cell in room.border_cells() {
                let mut tile =
                    crate::engine::map::tile::Tile::new("wall".to_string(), "wall".to_string());
                // tile.add_item(Item::Gold(Gold {
                //     value: 1,
                //     sprite_name: "gold".to_string(),
                // }));
                tile.cell_type = crate::engine::map::tile::CellType::Wall;

                map_builder.map_tiles.add_tile(cell.x, cell.y, tile);
            }
        }

        //connect rooms
        for i in 0..rooms.len() - 1 {
            let room1 = &rooms[i];
            let room2 = &rooms[i + 1];

            let mut corridor =
                crate::engine::level::corridor::Corridor::connect_rooms(room1, room2);

            for cell in corridor.cells() {
                let mut tile =
                    crate::engine::map::tile::Tile::new("test".to_string(), "test".to_string());
                tile.cell_type = crate::engine::map::tile::CellType::Floor;
                map_builder.map_tiles.add_tile(cell.x, cell.y, tile);
            }
        }

        map_builder.rooms = rooms;

        map_builder
    }
}
