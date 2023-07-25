#![allow(dead_code)]
use std::ops::Range;

pub mod corridor;

use slotmap::{new_key_type, SlotMap};

use rand::Rng;

use super::{
    core::world::World,
    map::{cell::Cell, Map},
};

new_key_type! { pub struct RoomKey; }

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Dimension {
    w: u16,
    h: u16,
}

impl Dimension {
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Room {
    top_left: Cell,
    width: u16,
    height: u16,
    id: RoomKey,
}

impl Room {
    fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            top_left: Cell::new(x, y),
            width,
            height,
            id: RoomKey::default(),
        }
    }

    // iterator over border cells
    pub fn border_cells(&self) -> Vec<Cell> {
        let mut cells = Vec::new();

        for x in self.top_left.x..self.top_left.x + self.width {
            cells.push(Cell::new(x, self.top_left.y));
            cells.push(Cell::new(x, self.top_left.y + self.height - 1));
        }

        for y in self.top_left.y..self.top_left.y + self.height {
            cells.push(Cell::new(self.top_left.x, y));
            cells.push(Cell::new(self.top_left.x + self.width - 1, y));
        }

        cells
    }

    // iterator over interior cells
    pub fn interior_cells(&self) -> Vec<Cell> {
        let mut cells = Vec::new();

        for x in self.top_left.x + 1..self.top_left.x + self.width - 1 {
            for y in self.top_left.y + 1..self.top_left.y + self.height - 1 {
                cells.push(Cell::new(x, y));
            }
        }

        cells
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.top_left.x <= other.top_left.x + other.width
            && self.top_left.x + self.width >= other.top_left.x
            && self.top_left.y <= other.top_left.y + other.height
            && self.top_left.y + self.height >= other.top_left.y
    }

    pub fn center(&self) -> Cell {
        println!("center: {:?}", self);
        let center_x = self.top_left.x + (self.width / 2);
        let center_y = self.top_left.y + (self.height / 2);

        Cell::new(center_x, center_y)
    }

    pub fn create_random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let w = rng.gen_range(5..15);
        let h = rng.gen_range(5..15);

        Self::new(x as u16, y as u16, w, h)
    }

    pub fn create_random_in_rect(
        top_left: Cell,
        size: Dimension,
        room_size_range: (Range<u16>, Range<u16>),
    ) -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(top_left.x..top_left.x + size.w);
        let y = rng.gen_range(top_left.y..top_left.y + size.h);

        let w = rng.gen_range(room_size_range.0);
        let h = rng.gen_range(room_size_range.1);

        Self::new(x, y, w, h)
    }
}

struct Level {
    map: Map,
    world: World,
    rooms: SlotMap<RoomKey, Room>,
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_room() {
        let mut rooms = SlotMap::with_key();
        let room1 = Room::new(0, 0, 10, 10);

        let id: RoomKey = rooms.insert(room1);

        let r = rooms.get_mut(id).unwrap();
        r.id = id;

        assert_eq!(id, r.id);
    }
}
