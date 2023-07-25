use std::fmt::Display;
use std::mem::swap;
use std::ops::{AddAssign, MulAssign, Sub, SubAssign};

use num_traits::Signed;

use super::cell::Cell;

/// Returns a vector of cells that form a line between two points using Bresenham's line algorithm.
///
/// # Arguments
///
/// * `x0` - The x-coordinate of the starting point.
/// * `y0` - The y-coordinate of the starting point.
/// * `x1` - The x-coordinate of the ending point.
/// * `y1` - The y-coordinate of the ending point.
///
/// # Examples
///
/// ```
/// use crate::engine::map::bresenham::line;
///
/// let cells = line(0, 0, 3, 3);
/// assert_eq!(cells, vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
///
/// reference: https://phinjensen.com/blog/2022/rustyrender-bresenhams-line-drawing-algorithm/
/// ```
pub fn line<
    T: Sub
        + Signed
        + PartialOrd
        + MulAssign
        + AddAssign
        + SubAssign
        + From<i32>
        + Copy
        + Display
        + std::fmt::Debug,
>(
    mut x0: T,
    mut y0: T,
    mut x1: T,
    mut y1: T,
) -> Vec<(T, T)> {
    let steep = (x0 - x1).abs() < (y0 - y1).abs();
    // let reverse_output = x0 > x1;
    let start_x = x0;
    let start_y = y0;
    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2: T = dy.abs() * T::from(2);
    let mut error2 = T::from(0);
    let mut y = y0;

    let mut cells: Vec<(T, T)> = vec![];

    let mut x = x0;
    while x <= x1 {
        if steep {
            // image.set(y as usize, x as usize, color).ok();
            cells.push((y, x));
        } else {
            // image.set(x as usize, y as usize, color).ok();
            cells.push((x, y));
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { T::from(1) } else { T::from(-1) };
            error2 -= dx * T::from(2);
        }
        x += T::from(1);
    }
    println!("cells: {:?}", cells);
    if cells[0].0 != start_x && cells[0].1 != start_y {
        println!("cells[0] != x0 || cells[0] != y0");
        cells.reverse();
    }
    cells
}

/// Returns a vector of cells that form a line between the start and end cells using Bresenham's line algorithm.
///
/// # Arguments
///
/// * `start` - A reference to the starting cell.
/// * `end` - A reference to the ending cell.
///
/// # Examples
///
/// ```
/// use crate::engine::map::bresenham::line_to_cell;
/// use crate::engine::map::Cell;
///
/// let start = Cell::new(0, 0);
/// let end = Cell::new(3, 3);
///
/// let line = line_to_cell(&start, &end);
///
/// assert_eq!(line.len(), 4);
/// assert_eq!(line[0], Cell::new(0, 0));
/// assert_eq!(line[1], Cell::new(1, 1));
/// assert_eq!(line[2], Cell::new(2, 2));
/// assert_eq!(line[3], Cell::new(3, 3));
/// ```
pub fn line_to_cell(start: &Cell, end: &Cell) -> Vec<Cell> {
    let cells = line(start.x as i32, start.y as i32, end.x as i32, end.y as i32);
    cells
        .iter()
        .map(|c| Cell::new(c.0 as u16, c.1 as u16))
        .collect()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_line() {
        let cells = line(0, 0, 3, 3);
        assert_eq!(cells, vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn test_line_2() {
        let cells = line(0, 0, 3, 0);
        assert_eq!(cells, vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
    }
}
