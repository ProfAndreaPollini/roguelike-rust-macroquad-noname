use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

mod dimension;
mod direction;
//mod linearize;
mod camera;
mod entity;
mod grid;
mod map;
mod render;

mod scenes;
mod sprite;
mod vector;

pub use camera::{Camera, Camera2D, Viewport};
pub use dimension::*;
pub use entity::action::*;
pub use entity::*;
pub use grid::*;
pub use map::*;
pub use render::*;
pub use scenes::*;
pub use sprite::*;
pub use vector::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Cast<T> {
    fn cast(self) -> T;
}

pub trait Scalar:
    'static
    + Sized
    + Copy
    + Debug
    + Zero
    + One
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + PartialOrd
{
}

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

impl Scalar for i32 {}
impl Zero for i32 {
    const ZERO: Self = 0;
}
impl One for i32 {
    const ONE: Self = 1;
}

impl Cast<i32> for i32 {
    fn cast(self) -> i32 {
        self
    }
}

impl Scalar for u32 {}
impl Zero for u32 {
    const ZERO: Self = 0;
}
impl One for u32 {
    const ONE: Self = 1;
}

impl Cast<u32> for i32 {
    fn cast(self) -> u32 {
        self as u32
    }
}

impl Scalar for usize {}
impl Zero for usize {
    const ZERO: Self = 0;
}
impl One for usize {
    const ONE: Self = 1;
}

impl Cast<i32> for usize {
    fn cast(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vec2;
    use crate::vector::Vector2;

    #[test]
    fn it_works() {
        let v = Vector2::new(1, 2);

        assert_eq!(v.x(), 1);
    }
}
