use crate::{Dimension2D, IntVector2, MapBuilder, MapBuilderAlgorithm, Room, Tile, Vec2};

use rand::Rng;

#[derive(Debug, Clone)]
pub struct RoomBuilder<T>
where
    T: Tile,
{
    _marker: std::marker::PhantomData<T>,
}

impl<T> RoomBuilder<T>
where
    T: Tile,
{
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    fn connect_rooms(
        &self,
        map_builder: &MapBuilder<T>,
        room1: &Room,
        room2: &Room,
    ) -> Vec<IntVector2> {
        let mut rng = rand::thread_rng();

        let mut start = room1.center();
        let mut end = room2.center();

        let path = map_builder.map.line(start, end);

        let mut corridor = Vec::<IntVector2>::new();

        for cell in path {
            // if map_builder.map.get(cell.x(), cell.y()).is_some() {
            corridor.push(cell);
            // }
        }

        // let mut corridor = Room::new(room1.center(), Dimension2D::new(0, 0));

        // let mut current_pos = room1.center();

        // while current_pos != room2.center() {
        //     let mut next_pos = current_pos;

        //     let direction = if rng.gen_range(0..2) == 1 {
        //         if current_pos.x() < room2.center().x() {
        //             "right"
        //         } else {
        //             "left"
        //         }
        //     } else {
        //         if current_pos.y() < room2.center().y() {
        //             "down"
        //         } else {
        //             "up"
        //         }
        //     };

        //     match *direction {
        //         "up" => *next_pos.y_mut() -= 1,
        //         "down" => *next_pos.y_mut() += 1,
        //         "left" => *next_pos.x_mut() -= 1,
        //         "right" => *next_pos.x_mut() += 1,
        //         _ => {}
        //     }

        //     corridor = Room::new(current_pos, Dimension2D::new(1, 1));

        //     current_pos = next_pos;
        // }

        corridor
    }
}

impl<T> Default for RoomBuilder<T>
where
    T: Tile,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tile> MapBuilderAlgorithm<T> for RoomBuilder<T> {
    fn build<'a>(&self, map_builder: &'a mut MapBuilder<T>) -> &'a mut MapBuilder<T> {
        let mut rooms = Vec::<Room>::new();

        // let mut room = Room::create_random(20, 20);
        // self.rooms.push(room);

        // room = Room::create_random_in_rect(
        //     IntVector2::new(12, 12),
        //     Dimension2D::<usize>::new(20, 20),
        //     (5..15, 5..15),
        // );
        // self.rooms.push(room);

        let mut attempts = 0;
        let map_extent = map_builder.map.size();
        while rooms.len() < 10 && attempts < 1000 {
            let candidate = Room::create_random_in_rect(
                IntVector2::new(map_extent.left(), map_extent.top()),
                Dimension2D::<usize>::new(map_extent.width(), map_extent.height()),
                (10..25, 10..25),
            );
            // for room in rooms.iter() {
            //     println!("candidate: {:?}", candidate);
            //     if candidate.intersects(room) {
            //         attempts += 1;
            //         continue;
            //     }
            // }
            if rooms.iter().all(|room| !candidate.intersects(room)) {
                rooms.push(candidate);
            } else {
                attempts += 1;
            }
        }

        rooms.iter().for_each(|room| {
            room.cells().iter().for_each(|pos| {
                let tile = map_builder.tiles.get("floor").unwrap().clone();

                map_builder.map.set(pos.x(), pos.y(), tile);
            });
        });

        rooms.iter().for_each(|room| {
            room.border_cells().iter().for_each(|pos| {
                let tile = map_builder.tiles.get("wall").unwrap().clone();

                map_builder.map.set(pos.x(), pos.y(), tile);
            });
            map_builder.rooms.push(room.clone());
        });

        //connect rooms
        for i in 0..rooms.len() - 1 {
            let room1 = &rooms[i];
            let room2 = &rooms[i + 1];

            let corridor = self.connect_rooms(map_builder, room1, room2);
            let mut rng = rand::thread_rng();
            for cell in corridor.iter() {
                let tile = map_builder.tiles.get("floor").unwrap().clone();
                let wall_tile = map_builder.tiles.get("wall").unwrap().clone();
                let offset = 3; //rng.gen_range(0..5);
                for i in -offset + 1..offset {
                    map_builder.map.set(cell.x() + i, cell.y(), tile.clone());
                }

                // map_builder.map.set(cell.x(), cell.y(), tile);
            }
        }

        map_builder
    }
}
