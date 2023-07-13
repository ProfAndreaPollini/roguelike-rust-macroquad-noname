#![allow(dead_code, unused_variables)]
use macroquad::prelude::IVec2;

use super::map::Map;

// #[derive(Clone, Copy, Debug, PartialEq)]
// struct Octant {
//     number: u8,
//     origin: (i16, i16),
// }

// impl Octant {
//     fn new(number: u8, origin: (i16, i16)) -> Self {
//         Self { number, origin }
//     }

//     fn transform(&self, x: i16, y: i16) -> IVec2 {
//         let p = match self.number {
//             0 => (x, y),
//             1 => (y, x),
//             2 => (y, -x),
//             3 => (x, -y),
//             4 => (-x, -y),
//             5 => (-y, -x),
//             6 => (-y, x),
//             7 => (-x, y),
//             _ => panic!("Invalid octant number"),
//         };
//         IVec2::new((self.origin.0 + p.0) as i32, (self.origin.1 + p.1) as i32)
//     }
// }

// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct Row {
//     pub depth: i16,
//     pub start: i16,
//     pub end: i16,
// }

// impl Row {
//     fn new(depth: i16, start: i16, end: i16) -> Self {
//         Self { depth, start, end }
//     }

//     pub fn tiles(&self) -> Vec<(i16, i16)> {
//         let mut tiles = Vec::new();
//         let min_col = self.start * self.depth;
//         let max_col = self.end * self.depth;
//         for x in min_col..=max_col {
//             tiles.push((self.depth, x));
//         }
//         tiles
//     }

//     pub fn next(&self) -> Self {
//         Self::new(self.depth + 1, self.start, self.end)
//     }
// }

// fn slope(tile: (i16, i16)) -> (i16, i16) {
//     let (col, row_depth) = tile;
//     (2 * col - 1, 2 * row_depth)
// }

// fn is_wall(map: &Map, x: u16, y: u16) -> bool {
//     match map.tile_at(x, y) {
//         Some(tile) => tile.cell_type == CellType::Wall,
//         None => false,
//     }
// }

// fn is_symmetric(row: Row, tile: (i16, i16)) -> bool {
//     let (row_depth, col) = tile;
//     (col >= row.depth * row.start && col <= row.depth * row.end)
// }

// pub fn shadowcast_fov(map: &Map, row: Row) -> Vec<(u16, u16)> {
//     let mut visible_tiles = Vec::new();
//     let mut rows = vec![row];

//     while !rows.is_empty() {
//         let row = rows.pop().unwrap();
//         let prev_tile: Option<(i16, i16)> = None;

//         for tile in row.tiles() {
//             let (x, y) = tile;
//             if is_wall(map, x as u16, y as u16) {
//                 if let Some(prev_tile) = prev_tile {
//                     rows.push(row.next());
//                     rows.push(Row::new(row.depth, prev_tile.0, tile.0 - 1));
//                 }
//                 break;
//             }
//             visible_tiles.push((x as u16, y as u16));
//         }
//     }

//     visible_tiles
// }

// struct Vec {
//     x: i32,
//     y: i32,
// }

// struct Stage {
//     // Implementa i dettagli della classe Stage
// }

// struct Fov<'a> {
//     stage: &'a Stage,
//     shadows: Vec<Shadow>,
// }

// impl<'a> Fov<'a> {
//     const MAX_VIEW_DISTANCE: f32 = 24.0;

//     const OCTANT_COORDINATES: [(Vec, Vec); 8] = [
//         (Vec { x: 0, y: -1 }, Vec { x: 1, y: 0 }),
//         (Vec { x: 1, y: 0 }, Vec { x: 0, y: -1 }),
//         (Vec { x: 1, y: 0 }, Vec { x: 0, y: 1 }),
//         (Vec { x: 0, y: 1 }, Vec { x: 1, y: 0 }),
//         (Vec { x: 0, y: 1 }, Vec { x: -1, y: 0 }),
//         (Vec { x: -1, y: 0 }, Vec { x: 0, y: 1 }),
//         (Vec { x: -1, y: 0 }, Vec { x: 0, y: -1 }),
//         (Vec { x: 0, y: -1 }, Vec { x: -1, y: 0 }),
//     ];

//     fn new(stage: &'a Stage) -> Self {
//         Fov {
//             stage,
//             shadows: Vec::new(),
//         }
//     }

//     fn refresh(&mut self, pos: Vec) {
//         if self.stage.game.hero.blindness.is_active() {
//             self.hide_all();
//             return;
//         }

//         for octant in 0..8 {
//             self.refresh_octant(pos, octant);
//         }

//         self.stage.set_visibility(pos, false, 0);
//     }

//     fn hide_all(&mut self) {
//         for pos in self.stage.bounds() {
//             self.stage.set_visibility(pos, true, 0);
//         }

//         self.stage
//             .set_visibility(self.stage.game.hero.pos, false, 0);
//     }

//     fn refresh_octant(&mut self, start: Vec, octant: usize) {
//         let row_inc = Self::OCTANT_COORDINATES[octant].0;
//         let col_inc = Self::OCTANT_COORDINATES[octant].1;

//         self.shadows.clear();

//         let bounds = self.stage.bounds();
//         let mut full_shadow = false;

//         let mut row = 1;
//         loop {
//             let pos = start + (row_inc * row);

//             if !bounds.contains(&pos) {
//                 break;
//             }

//             let mut past_max_distance = false;

//             for col in 0..=row {
//                 let mut fall_off = 255;

//                 if full_shadow || past_max_distance {
//                     self.stage.set_visibility(pos, true, fall_off);
//                 } else {
//                     fall_off = 0;
//                     let distance = (start - pos).length();
//                     if distance > Self::MAX_VIEW_DISTANCE {
//                         fall_off = 255;
//                         past_max_distance = true;
//                     } else {
//                         let normalized = distance / Self::MAX_VIEW_DISTANCE;
//                         let normalized = normalized * normalized;
//                         fall_off = (normalized * 255.0) as u8;
//                     }

//                     let projection = Self::get_projection(col, row);
//                     self.stage
//                         .set_visibility(pos, self.is_in_shadow(&projection), fall_off);

//                     if self.stage[pos].blocks_view {
//                         full_shadow = self.add_shadow(projection);
//                     }
//                 }

//                 pos += col_inc;

//                 if !bounds.contains(&pos) {
//                     break;
//                 }
//             }

//             row += 1;
//         }
//     }

//     fn get_projection(col: usize, row: usize) -> Shadow {
//         let top_left = col as f32 / (row as f32 + 2.0);
//         let bottom_right = (col + 1) as f32 / (row + 1) as f32;

//         Shadow::new(top_left, bottom_right)
//     }

//     fn is_in_shadow(&self, projection: &Shadow) -> bool {
//         for shadow in &self.shadows {
//             if shadow.contains(projection) {
//                 return true;
//             }
//         }

//         false
//     }

//     fn add_shadow(&mut self, shadow: Shadow) -> bool {
//         let mut index = 0;

//         while index < self.shadows.len() {
//             if self.shadows[index].start > shadow.start {
//                 break;
//             }
//             index += 1;
//         }

//         let overlaps_prev = index > 0 && self.shadows[index - 1].end > shadow.start;
//         let overlaps_next = index < self.shadows.len() && self.shadows[index].start < shadow.end;

//         if overlaps_next {
//             if overlaps_prev {
//                 self.shadows[index - 1].end =
//                     self.shadows[index - 1].end.max(self.shadows[index].end);
//                 self.shadows.remove(index);
//             } else {
//                 self.shadows[index].start = self.shadows[index].start.min(shadow.start);
//             }
//         } else if overlaps_prev {
//             self.shadows[index - 1].end = self.shadows[index - 1].end.max(shadow.end);
//         } else {
//             self.shadows.insert(index, shadow);
//         }

//         self.shadows.len() == 1 && self.shadows[0].start == 0.0 && self.shadows[0].end == 1.0
//     }
// }

// struct Shadow {
//     start: f32,
//     end: f32,
// }

// impl Shadow {
//     fn new(start: f32, end: f32) -> Self {
//         Shadow { start, end }
//     }

//     fn contains(&self, projection: &Shadow) -> bool {
//         self.start <= projection.start && self.end >= projection.end
//     }
// }

// Implementa i dettagli delle altre classi e funzioni utilizzate

enum Quadrant {
    North(IVec2),
    East(IVec2),
    South(IVec2),
    West(IVec2),
}

impl Quadrant {
    pub fn transform(&self, col: i32, row: i32) -> IVec2 {
        match self {
            Quadrant::North(origin) => IVec2::new(origin.x + col, origin.y - row),
            Quadrant::East(origin) => IVec2::new(origin.x + row, origin.y + col),
            Quadrant::South(origin) => IVec2::new(origin.x + col, origin.y + row),
            Quadrant::West(origin) => IVec2::new(origin.x - row, origin.y + col),
        }
    }
}

/*
A Row represents a segment of tiles bound between a start and end slope. depth represents the distance between the row and the quadrant’s origin.
*/
#[derive(Debug, Clone, Copy)]
struct Row {
    depth: i32,
    start: f32,
    end: f32,
}

impl Row {
    pub fn new(depth: i32, start: f32, end: f32) -> Self {
        Self { depth, start, end }
    }

    pub fn tiles(&self) -> Vec<IVec2> {
        let mut tiles = Vec::new();
        let min_col = round_ties_up(self.start * self.depth as f32);
        let max_col = round_ties_down(self.end * self.depth as f32);
        for x in min_col..=max_col {
            tiles.push(IVec2::new(x, self.depth));
        }
        tiles
    }

    pub fn next(&self) -> Self {
        Self::new(self.depth + 1, self.start, self.end)
    }
}

/*
round_ties_up and round_ties_down round n to the nearest integer. If n ends in .5, round_ties_up rounds up and round_ties_down rounds down.
*/

fn round_ties_up(n: f32) -> i32 {
    (n - 0.5).ceil() as i32
}

fn round_ties_down(n: f32) -> i32 {
    (n + 0.5).floor() as i32
}

pub fn compute_fov(map: &mut Map, start_pos: IVec2, fov_distance: i32) {
    map.set_tile_visible(start_pos.x as u16, start_pos.y as u16, false);

    // loop over the quadrants
    for i in 0..4 {
        let mut quadrant = match i {
            0 => Quadrant::North(start_pos),
            1 => Quadrant::East(start_pos),
            2 => Quadrant::South(start_pos),
            3 => Quadrant::West(start_pos),
            _ => panic!("Invalid quadrant number"),
        };
        println!("--------");
        // let mut quadrant = Quadrant::North(start_pos);
        let mut first_row = Row::new(1, -1., 1.);
        scan_iter(&mut first_row, &mut quadrant, map, fov_distance);
        println!("--------");
    }

    // let mut quadrant = Quadrant::North(start_pos);
}

fn scan_iter(start_row: &mut Row, quadrant: &mut Quadrant, map: &mut Map, fov_distance: i32) {
    let mut rows = vec![*start_row];
    while !rows.is_empty() {
        println!("rows {:?}", rows);
        let mut row = rows.pop().unwrap();
        if row.depth > fov_distance {
            return;
        }

        println!(">> row {:?} tiles {:?}", row, row.tiles());

        let mut prev_tile: Option<IVec2> = None;
        for tile in row.tiles() {
            let (x, y) = (tile.x, tile.y);
            println!(
                "tile {:?} | is wall? {:?} | is symmetric {:?}",
                tile,
                is_wall(map, quadrant, Some(tile)),
                is_symmetric(&row, quadrant, Some(tile))
            );
            if is_wall(map, quadrant, Some(tile)) || is_symmetric(&row, quadrant, Some(tile)) {
                println!("is wall or symmetric => reveal ");
                reveal(map, quadrant, Some(tile));
            }
            if is_wall(map, quadrant, prev_tile) && is_floor(map, quadrant, Some(tile)) {
                // let s = slope(tile);
                row.start = slope(tile);
                println!(
                    "prev was wall and current is floor =>new row start {:?}",
                    row.start
                );
            }
            if is_floor(map, quadrant, prev_tile) && is_wall(map, quadrant, Some(tile)) {
                let mut next_row = row.next();
                // let s = slope(tile);
                next_row.end = slope(tile);
                rows.push(next_row);
                println!(
                    "prev was floor and current is wall => new row {:?}",
                    next_row
                );
            }
            prev_tile = Some(tile);
        }
        if is_floor(map, quadrant, prev_tile) {
            let next_row = row.next();
            rows.push(next_row);
            println!("prev is floor => new row push {:?}", next_row);
        }
    }
}

// fn scan(row: &mut Row, quadrant: &mut Quadrant, map: &mut Map, fov_distance: i32) {
//     let prev_tile: Option<IVec2> = None;
//     println!("row tiles{:?}", row.tiles());
//     for tile in row.tiles() {
//         let (x, y) = (tile.x, tile.y);
//         if is_wall(map, quadrant, Some(tile)) || is_symmetric(row, quadrant, Some(tile)) {
//             reveal(map, quadrant, Some(tile));
//         }
//         if is_wall(map, quadrant, prev_tile) && is_floor(map, quadrant, Some(tile)) {
//             // let s = slope(tile);
//             row.start = slope(tile);
//         }

//         if is_floor(map, quadrant, prev_tile) && is_wall(map, quadrant, Some(tile)) {
//             let mut next_row = row.next();
//             // let s = slope(tile);
//             next_row.end = slope(tile);
//             scan(&mut next_row, quadrant, map, fov_distance);
//         }

//         println!("tile {:?} | row = {:?}", tile, row);
//         println!(
//             "is_wall = {} is floor = {}",
//             is_wall(map, quadrant, Some(tile)),
//             is_floor(map, quadrant, Some(tile))
//         );
//         println!(
//             "prev_tile is_wall = {} is floor = {}",
//             is_wall(map, quadrant, prev_tile),
//             is_floor(map, quadrant, prev_tile)
//         );

//         let prev_tile = tile;

//         // map.set_tile_visible(x as u16, y as u16, true);
//     }
//     if is_floor(map, quadrant, prev_tile) {
//         let mut next_row = row.next();

//         scan(&mut next_row, quadrant, map, fov_distance);
//     }
// }

fn is_floor(map: &mut Map, quadrant: &mut Quadrant, tile: Option<IVec2>) -> bool {
    if let Some(t) = tile {
        let coords = quadrant.transform(t.x, t.y);
        if let Some(tile) = map.tile_at(t.x as u16, t.y as u16) {
            !map.is_position_blocked(coords.x as u16, coords.y as u16)
        } else {
            false
        }
    } else {
        false
    }
}

fn reveal(map: &mut Map, quadrant: &Quadrant, tile: Option<IVec2>) {
    if let Some(t) = tile {
        let coords = quadrant.transform(t.x, t.y);
        if let Some(tile) = map.tile_at(coords.x as u16, coords.y as u16) {
            println!("reveal (transformed) {:?}", coords);
            map.set_tile_visible(coords.x as u16, coords.y as u16, true);
        }
    }
}

fn is_wall(map: &mut Map, quadrant: &Quadrant, tile: Option<IVec2>) -> bool {
    if let Some(t) = tile {
        let coords = quadrant.transform(t.x, t.y);
        if let Some(tile) = map.tile_at(t.x as u16, t.y as u16) {
            map.is_position_blocked(coords.x as u16, coords.y as u16)
        } else {
            false
        }
    } else {
        false
    }
}

/*
slope calculates new start and end slopes.
It’s used in two situations: [1], if prev_tile (on the left) was a wall tile and tile (on the right) is a floor tile,
 then the slope represents a start slope and should be tangent to the right edge of the wall tile.
[2], if prev_tile was a floor tile and tile is a wall tile,
 then the slope represents an end slope and should be tangent to the left edge of the wall tile.
In both situations, the line is tangent to the left edge of the current tile,
so we can use a single slope function for both start and end slopes.
*/

fn slope(tile: IVec2) -> f32 {
    let (col, row_depth) = (tile.x, tile.y);
    let dx = 2. * col as f32 - 1.;
    let dy = 2. * row_depth as f32;
    dx / dy
}

/*
is_symmetric checks if a given floor tile can be seen symmetrically from the origin. It returns true if the central point of the tile is in the sector swept out by the row’s start and end slopes. Otherwise, it returns false.
*/

fn is_symmetric(row: &Row, quadrant: &Quadrant, tile: Option<IVec2>) -> bool {
    if let Some(t) = tile {
        let (col, row_depth) = (t.x, t.y);
        // let (start_slope, end_slope) = (
        //     slope(quadrant.transform(col, row_depth)),
        //     slope(quadrant.transform(col, row_depth)),
        // );
        let row_depth = row.depth as f32;
        (t.x as f32 >= row.start * row_depth) && (t.x as f32 <= row.end * row_depth)
        //(t.x as f32 > row.start * row_depth) && ((t.x as f32) < row.end * row_depth)
    } else {
        false
    }
}
