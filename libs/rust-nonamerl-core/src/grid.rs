use std::{
    collections::HashMap,
    fmt::{self, Display},
    hash::Hash,
    mem::swap,
    ops::{AddAssign, MulAssign, Sub, SubAssign},
};

use morton_encoding::morton_encode;
use num_traits::Signed;

use crate::{
    dimension::{Dimension2, Dimension2D},
    vector::{IntVector2, Vec2},
    Scalar,
};

pub trait Plane<P, T> {
    fn at(&self, pos: P) -> Option<&T>;
    fn at_mut(&mut self, pos: P) -> Option<&mut T>;
    fn put(&mut self, pos: P, value: T);
    fn line(&self, start: P, end: P) -> Vec<P>;
    fn neighbors(&self, pos: P) -> Vec<P>;
}

// pub trait Lattice2D<T>: Plane<IntVector2, T> {
//     type Encoder: PositionEncoder;
// }

pub trait PositionEncoder {
    // type Encoder: Copy + Display + Sub<Output = Self::Encoder> + AddAssign + MulAssign + SubAssign;

    fn encode(&self, v: IntVector2) -> u64;
    fn decode(&self, val: u64) -> IntVector2;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntVector2Encoder {}

fn translate(v: i32) -> u32 {
    // add the max i32 to v
    // so that v is always positive
    // and we can use the full range of u64
    let v = v as i64;
    (v + i32::MAX as i64) as u32
}

fn untranslate(v: u32) -> i32 {
    // subtract the max i32 from v
    // so that v is always positive
    // and we can use the full range of u64
    let v = v as i64;
    (v - i32::MAX as i64) as i32
}

impl PositionEncoder for IntVector2Encoder {
    fn encode(&self, v: IntVector2) -> u64 {
        // (v.x() + v.y() * 1000) as u32
        morton_encode([translate(v.x()), translate(v.y())])
    }

    fn decode(&self, val: u64) -> IntVector2 {
        let [x, y] = morton_encoding::morton_decode(val);
        IntVector2::new(untranslate(x), untranslate(y))
    }
}

// pub struct MortonEncoder<V>
// where
//     V: Vec2,
// {
//     _phantom: std::marker::PhantomData<V>,
// }

pub trait InsertKeyValue<K, T> {
    fn insert_key_value(&mut self, key: K, value: T);
    fn get_key_value(&self, key: K) -> Option<&T>;
}

impl<K: Eq + Hash, T> InsertKeyValue<K, T> for HashMap<K, T> {
    fn insert_key_value(&mut self, key: K, value: T) {
        self.insert(key, value);
    }

    fn get_key_value(&self, key: K) -> Option<&T> {
        self.get(&key)
    }
}

// pub trait IntegerGrid<T>: Lattice2D<T> + Plane<IntVector2, T> {
//     type Encoder: PositionEncoder;
// }

#[derive(Clone, Debug)]
pub struct LatticeGrid2D<T>
where
    T: Clone,
{
    encoder: IntVector2Encoder,
    data: HashMap<u64, T>,
}

impl<T: Clone> std::fmt::Display for LatticeGrid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("LatticeGrid2D:");
        write!(f, "{}", s)
    }
}

impl<T: Clone> LatticeGrid2D<T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn new() -> Self {
        let data = HashMap::new();
        let encoder = IntVector2Encoder {};

        Self { data, encoder }
    }

    pub fn line(&self, start: IntVector2, end: IntVector2) -> Vec<IntVector2> {
        bresenham_line(start.x(), start.y(), end.x(), end.y())
            .iter()
            .map(|(x, y)| IntVector2::new(*x, *y))
            .collect()
    }

    pub fn put(&mut self, pos: IntVector2, value: T) {
        self.data.insert(self.encoder.encode(pos), value);
    }

    pub fn at(&self, pos: IntVector2) -> Option<&T> {
        self.data.get(&self.encoder.encode(pos))
    }

    fn at_mut(&mut self, pos: IntVector2) -> Option<&mut T> {
        self.data.get_mut(&self.encoder.encode(pos))
    }

    fn neighbors(&self, pos: IntVector2) -> Vec<IntVector2> {
        let mut neighbors = Vec::new();
        let [x, y] = pos.as_array();

        neighbors.push(IntVector2::new(x - 1, y));

        neighbors.push(IntVector2::new(x + 1, y));

        neighbors.push(IntVector2::new(x, y - 1));

        neighbors.push(IntVector2::new(x, y + 1));

        neighbors
    }
}

// #[derive(Debug, Clone, Default)]
// struct DummyEncoder {}

// impl PositionEncoder for DummyEncoder {
//     fn encode(&self, v: IntVector2) -> u32 {
//         (v.x() + v.y() * 1000) as u32
//     }

//     fn decode(&self, val: u32) -> IntVector2 {
//         IntVector2::new(val % 1000, val / 1000)
//     }
// }

pub fn bresenham_line<T: Scalar + Signed + From<i32>>(
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
    // println!("cells: {:?}", cells);
    if cells[0].0 != start_x && cells[0].1 != start_y {
        // println!("cells[0] != x0 || cells[0] != y0");
        cells.reverse();
    }
    cells
}

#[cfg(test)]

mod tests {

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    struct Test {}

    #[test]
    fn test_grid2d() {
        let mut grid = LatticeGrid2D::<Test>::new();
        grid.put(IntVector2::new(0, 0), Test {});
        grid.put(IntVector2::new(1, 1), Test {});

        assert_eq!(grid.at(IntVector2::new(0, 0)), Some(&Test {}));
        assert_eq!(grid.at(IntVector2::new(10, 10)), None);
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(0), 2147483647);
        assert_eq!(translate(1), 2147483648);
        assert_eq!(translate(-1), 2147483646);
        assert_eq!(translate(2147483647), 4294967294);
        assert_eq!(translate(-2147483647), 0);
    }

    #[test]
    fn test_untranslate() {
        assert_eq!(untranslate(2147483647), 0);
        assert_eq!(untranslate(2147483648), 1);
        assert_eq!(untranslate(2147483646), -1);
        assert_eq!(untranslate(4294967294), 2147483647);
        assert_eq!(untranslate(0), -2147483647);
    }

    #[test]
    fn test_translate_untranslate() {
        assert_eq!(untranslate(translate(0)), 0);
        assert_eq!(untranslate(translate(1)), 1);
        assert_eq!(untranslate(translate(-1)), -1);
        assert_eq!(untranslate(translate(2147483647)), 2147483647);
        assert_eq!(untranslate(translate(-2147483648)), -2147483648);
    }
}
