use rand::{thread_rng, Rng};

use crate::engine::map::{bresenham::line, cell::Cell};

use super::Room;

#[derive(Debug)]
pub enum CorridorType {
    Simple,
    Directional,
}

#[derive(Debug)]
pub struct Corridor {
    start_x: u16,
    start_y: u16,
    end_x: u16,
    end_y: u16,
    corridor_type: CorridorType,
    cells: Vec<Cell>,
}

impl Default for Corridor {
    fn default() -> Self {
        Self {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            corridor_type: CorridorType::Simple,
            cells: Vec::new(),
        }
    }
}

impl Corridor {
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    //build cells
    fn build_cells(&mut self) {
        println!("build_cells: {:?}", self);
        let mut rng = thread_rng();

        let mut x = self.start_x;
        let mut y = self.start_y;

        self.cells.push(Cell::new(x, y));

        let mut bresenham_cells = Vec::<(isize, isize)>::new();
        for c in line(
            x as isize,
            y as isize,
            self.end_x as isize,
            self.end_y as isize,
        ) {
            bresenham_cells.push(c);
        }

        for c in line(
            (x + 1) as isize,
            y as isize,
            self.end_x as isize,
            self.end_y as isize,
        ) {
            bresenham_cells.push(c);
        }

        for c in line(
            x as isize,
            (y + 1) as isize,
            self.end_x as isize,
            (self.end_y + 1) as isize,
        ) {
            bresenham_cells.push(c);
        }

        for c in bresenham_cells.iter() {
            self.cells.push(Cell::new(c.0 as u16, c.1 as u16));
        }
        // while x != self.end_x || y != self.end_y {
        //     if rng.gen_bool(0.5) {
        //         if x != self.end_x {
        //             x += 1;
        //         }
        //     } else if y != self.end_y {
        //         y += 1;
        //     }

        //     self.cells.push(Cell::new(x, y));
        // }

        println!("build_cells: {:?}", self);
    }

    pub fn connect_rooms(room1: &Room, room2: &Room) -> Self {
        let mut corridor = Corridor::default();

        let room1_center = room1.center();
        let room2_center = room2.center();

        // let mut rng = thread_rng();

        // if rng.gen_bool(0.5) {
        corridor.start_x = room1_center.x;
        corridor.start_y = room1_center.y;
        corridor.end_x = room2_center.x;
        corridor.end_y = room2_center.y;

        //     if rng.gen_bool(0.5) {
        //         corridor.end_x = room2_center.x;
        //         corridor.end_y = room2_center.y;
        //     }
        // } else {
        //     corridor.start_x = room1_center.x;
        //     corridor.start_y = room2_center.y;
        //     corridor.end_x = room2_center.x;
        //     corridor.end_y = room2_center.y;

        //     if rng.gen_bool(0.5) {
        //         corridor.end_x = room1_center.x;
        //         corridor.end_y = room1_center.y;
        //     }
        // }

        corridor.build_cells();

        corridor
    }
}
