use std::ops::Range;

use crate::{Dimension2, Dimension2D, IntVector2, Vec2};
use rand::Rng;
#[derive(Clone, Debug)]
pub struct Room {
    pos: IntVector2,
    size: Dimension2D<usize>,
}

impl Room {
    pub fn new(pos: IntVector2, size: Dimension2D<usize>) -> Self {
        Self { pos, size }
    }

    pub fn border_cells(&self) -> Vec<IntVector2> {
        let mut cells = Vec::<IntVector2>::new();

        for x in self.pos.x()..self.pos.x() + self.size.width() as i32 {
            cells.push(IntVector2::new(x, self.pos.y()));
            cells.push(IntVector2::new(
                x,
                self.pos.y() + self.size.height() as i32 - 1,
            ));
        }

        for y in self.pos.y()..self.pos.y() + self.size.height() as i32 {
            cells.push(IntVector2::new(self.pos.x(), y));
            cells.push(IntVector2::new(
                self.pos.x() + self.size.width() as i32 - 1,
                y,
            ));
        }

        cells
    }

    pub fn interior_cells(&self) -> Vec<IntVector2> {
        let mut cells = Vec::<IntVector2>::new();

        for x in self.pos.x() + 1..self.pos.x() + self.size.width() as i32 - 1 {
            for y in self.pos.y() + 1..self.pos.y() + self.size.height() as i32 - 1 {
                cells.push(IntVector2::new(x, y));
            }
        }

        cells
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.pos.x() <= other.pos.x() + other.size.width() as i32
            && self.pos.x() + self.size.width() as i32 >= other.pos.x()
            && self.pos.y() <= other.pos.y() + other.size.height() as i32
            && self.pos.y() + self.size.height() as i32 >= other.pos.y()
    }

    pub fn center(&self) -> IntVector2 {
        IntVector2::new(
            self.pos.x() + self.size.width() as i32 / 2,
            self.pos.y() + self.size.height() as i32 / 2,
        )
    }

    pub fn create_random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let w = rng.gen_range(5..15);
        let h = rng.gen_range(5..15);

        Self::new(IntVector2::new(x, y), Dimension2D::new(w, h))
    }

    pub fn create_random_in_rect(
        top_left: IntVector2,
        size: Dimension2D<usize>,
        room_size_range: (Range<u16>, Range<u16>),
    ) -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(top_left.x()..top_left.x() + size.width() as i32);
        let y = rng.gen_range(top_left.y()..top_left.y() + size.height() as i32);

        let w = rng.gen_range(room_size_range.0);
        let h = rng.gen_range(room_size_range.1);

        Self::new(
            IntVector2::new(x, y),
            Dimension2D::new(w as usize, h as usize),
        )
    }

    pub fn cells(&self) -> Vec<IntVector2> {
        let mut cells = Vec::<IntVector2>::new();

        for x in self.pos.x()..self.pos.x() + self.size.width() as i32 {
            for y in self.pos.y()..self.pos.y() + self.size.height() as i32 {
                cells.push(IntVector2::new(x, y));
            }
        }

        cells
    }
}

#[cfg(test)]

mod tests {

    #[test]
    fn test_room_intersects() {
        use super::*;

        let room1 = Room::new(IntVector2::new(0, 0), Dimension2D::new(10, 10));
        let room2 = Room::new(IntVector2::new(5, 5), Dimension2D::new(10, 10));
        let room3 = Room::new(IntVector2::new(11, 11), Dimension2D::new(10, 10));

        assert!(room1.intersects(&room2));
        assert!(!room1.intersects(&room3));
    }
}
