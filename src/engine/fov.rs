#![allow(dead_code, unused_variables)]
use macroquad::prelude::IVec2;

use crate::engine::map::bresenham::line;

use super::{
    core::direction::Direction,
    map::{bresenham::line_to_cell, cell::Cell, Map},
};

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

/// Computes the field of view (FOV) for the given map and starting position, up to the given distance.
///
/// # Arguments
///
/// * `map` - A mutable reference to the map to compute the FOV for.
/// * `start_pos` - The starting position to compute the FOV from.
/// * `fov_distance` - The maximum distance to compute the FOV up to.
///
/// # Algorithm
///
/// This function uses the shadowcasting algorithm to compute the FOV. The algorithm works by dividing the map into four quadrants, and scanning each quadrant separately. For each quadrant, the algorithm starts at the origin (the starting position), and scans each row of tiles in the quadrant, starting from the first row and moving outward. For each row, the algorithm computes the start and end slopes of the row, and then scans each tile in the row to determine if it is visible. If a tile is visible, it is marked as such in the map. The algorithm continues scanning rows until it reaches the maximum FOV distance or until it encounters a tile that blocks visibility (such as a wall). Once all four quadrants have been scanned, the FOV computation is complete.
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
        // debug!("--------");
        // let mut quadrant = Quadrant::North(start_pos);
        let mut first_row = Row::new(1, -1., 1.);
        scan_iter(&mut first_row, &mut quadrant, map, fov_distance);
        // debug!("--------");
    }

    // let mut quadrant = Quadrant::North(start_pos);
}

/// Scan the tiles in the given quadrant, starting from the given row, and reveal any tiles that are visible within the given field of view distance.
///
/// # Arguments
///
/// * `start_row` - A mutable reference to the starting row of tiles to scan.
/// * `quadrant` - A mutable reference to the quadrant of tiles to scan.
/// * `map` - A mutable reference to the map containing the tiles to scan.
/// * `fov_distance` - The maximum distance from the starting position that tiles can be revealed.
fn scan_iter(start_row: &mut Row, quadrant: &mut Quadrant, map: &mut Map, fov_distance: i32) {
    let mut rows = vec![*start_row];
    while let Some(mut row) = rows.pop() {
        // debug!("rows {:?}", rows);

        if row.depth > fov_distance {
            return;
        }

        // debug!(">> row {:?} tiles {:?}", row, row.tiles());

        let mut prev_tile: Option<IVec2> = None;
        for tile in row.tiles() {
            let (x, y) = (tile.x, tile.y);
            // debug!(
            //     "tile {:?} | is wall? {:?} | is symmetric {:?}",
            //     tile,
            //     is_wall(map, quadrant, Some(tile)),
            //     is_symmetric(&row, quadrant, Some(tile))
            // );
            if is_wall(map, quadrant, Some(tile)) || is_symmetric(&row, quadrant, Some(tile)) {
                // debug!("is wall or symmetric => reveal ");
                reveal(map, quadrant, Some(tile));
            }
            if is_wall(map, quadrant, prev_tile) && is_floor(map, quadrant, Some(tile)) {
                // let s = slope(tile);
                row.start = slope(tile);
                // debug!(
                //     "prev was wall and current is floor =>new row start {:?}",
                //     row.start
                // );
            }
            if is_floor(map, quadrant, prev_tile) && is_wall(map, quadrant, Some(tile)) {
                let mut next_row = row.next();
                // let s = slope(tile);
                next_row.end = slope(tile);
                rows.push(next_row);
                // debug!(
                //     "prev was floor and current is wall => new row {:?}",
                //     next_row
                // );
            }
            prev_tile = Some(tile);
        }
        if is_floor(map, quadrant, prev_tile) {
            let next_row = row.next();
            rows.push(next_row);
            // debug!("prev is floor => new row push {:?}", next_row);
        }
    }
}

/// Determines whether the tile at the given position is a floor tile.
///
/// # Arguments
///
/// * `map` - A mutable reference to the map.
/// * `quadrant` - A mutable reference to the quadrant.
/// * `tile` - An optional `IVec2` representing the position of the tile to check.
///
/// # Returns
///
/// A boolean indicating whether the tile at the given position is a floor tile.
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

/// Reveals the tile at the given position, setting it to be visible on the map.
///
/// # Arguments
///
/// * `map` - A mutable reference to the map.
/// * `quadrant` - A mutable reference to the quadrant.
/// * `tile` - An optional `IVec2` representing the position of the tile to reveal.
fn reveal(map: &mut Map, quadrant: &Quadrant, tile: Option<IVec2>) {
    if let Some(t) = tile {
        let coords = quadrant.transform(t.x, t.y);
        if let Some(tile) = map.tile_at(coords.x as u16, coords.y as u16) {
            // println!("reveal (transformed) {:?}", coords);
            map.set_tile_visible(coords.x as u16, coords.y as u16, true);
        }
    }
}

/// Determines whether the tile at the given position is a wall tile.
///
/// # Arguments
///
/// * `map` - A mutable reference to the map.
/// * `quadrant` - A mutable reference to the quadrant.
/// * `tile` - An optional `IVec2` representing the position of the tile to check.
///
/// # Returns
///
/// A boolean indicating whether the tile at the given position is a wall tile.
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

// BRESENHAM FOV ALGORITHM

pub fn bresenham_fov(
    start: &Cell,
    direction: &Direction,
    depth: u16,
    angle: f32,
    map: &mut Map,
) -> Vec<Cell> {
    let tan_angle = (angle * 0.5).to_radians().tan();
    let p_size = (depth as f32 * tan_angle).ceil() as u16;
    println!("p_size = {},  tan_angle={:?}", p_size, tan_angle);
    let mut candidates = Vec::<Cell>::new();

    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = 0;
    let mut end = Cell::new(0, 0);

    let start_x = start.x as i32;
    let start_y = start.y as i32;

    let mut direction_angle = match direction {
        Direction::Up => 0.0,
        Direction::Right => 90.0,
        Direction::Down => 180.0,
        Direction::Left => 270.0,
    };

    direction_angle -= 90.;

    let start_angle = direction_angle - angle * 0.5;
    let end_angle = direction_angle + angle * 0.5;

    // if start_angle < 0.0 {
    //     start_angle += 360.0;
    // }

    // if end_angle > 360.0 {
    //     end_angle -= 360.0;
    // }

    // println!("start_x = {}, start_y = {}", start_x, start_y);
    // println!("start_angle = {}, end_angle = {}", start_angle, end_angle);

    let start_pos_x = (start_x as f32 + depth as f32 * start_angle.to_radians().cos()) as i32;
    let start_pos_y = (start_y as f32 + depth as f32 * start_angle.to_radians().sin()) as i32;
    // println!("start_x = {}, start_y = {}", end_x, end_y);

    let end_pos_x = (start_x as f32 + depth as f32 * end_angle.to_radians().cos()) as i32;
    let end_pos_y = (start_y as f32 + depth as f32 * end_angle.to_radians().sin()) as i32;
    // println!("end_x = {}, end_y = {}", end_x, end_y);

    // println!(
    //     "start_pos_x = {}, start_pos_y = {} end_pos__x = {}, end_pos_y = {}",
    //     start_pos_x, start_pos_y, end_pos_x, end_pos_y
    // );

    let targets = line(start_pos_x, start_pos_y, end_pos_x, end_pos_y);

    let mut points = Vec::<(i32, i32)>::new();

    for target in targets {
        // println!(
        //     "dx = {}, dy = {} start =({},{}) end = ({},  {})",
        //     dx, dy, start_x, start_y, target.0, target.1
        // );
        let mut line_pos = line(start_x, start_y, target.0, target.1);
        // println!("line_pos = {:?}", line_pos);
        line_pos.retain(|&x| x.0 >= 0 && x.1 >= 0);
        // println!("line_pos after = {:?}", line_pos);
        for pos in line_pos {
            // check if the point is inside the map
            // if pos.0 < 0 || pos.1 < 0 {
            //     break;
            // }

            // check if the point is a blocking tile
            if map.is_position_blocked(pos.0 as u16, pos.1 as u16) {
                points.push(pos);
                break;
            }
            points.push(pos);
        }
    }

    // filter points to remove duplicates
    points.sort();
    points.dedup();

    // filter points with negative values

    // populate candidates from points
    for point in points {
        let cell = Cell::new(point.0 as u16, point.1 as u16);

        candidates.push(cell);
    }

    for cell in candidates.iter() {
        map.set_tile_visible(cell.x, cell.y, true);
    }
    candidates
}
