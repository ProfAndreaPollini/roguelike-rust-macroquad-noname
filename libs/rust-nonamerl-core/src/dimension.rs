use num_traits::NumCast;

use crate::{
    vector::Vector2,
    vector::{IntVector2, Vec2},
    Cast, Scalar,
};

pub trait Dimension2 {
    type Scalar: Scalar;

    fn width(&self) -> Self::Scalar;
    fn height(&self) -> Self::Scalar;
}

#[derive(Debug, Copy, Clone)]
/// A generic struct representing a dimension in 2D space.
pub struct Dimension2D<T: Scalar>(Vector2<T>);

impl<T: Scalar> Dimension2D<T> {
    pub fn new(width: T, height: T) -> Self {
        Self(Vector2::new(width, height))
    }
}

impl<T: Scalar> PartialEq for Dimension2D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.x() == other.0.x() && self.0.y() == other.0.y()
    }
}

impl<T: Scalar> Dimension2 for Dimension2D<T> {
    type Scalar = T;

    fn width(&self) -> Self::Scalar {
        self.0.x()
    }

    fn height(&self) -> Self::Scalar {
        self.0.y()
    }
}

#[derive(Debug, Copy, Clone)]
/// A generic struct representing a dimension in 2D space.
///
/// This struct is used to represent the size of a cell in a grid.
///
///
/// # Examples
///
///     
pub struct IntExtent2D(IntVector2, Dimension2D<usize>);

impl IntExtent2D {
    pub fn new(x: i32, y: i32, width: usize, height: usize) -> Self {
        Self(IntVector2::new(x, y), Dimension2D::new(width, height))
    }

    pub fn left(&self) -> i32 {
        self.0.x()
    }

    pub fn right(&self) -> i32 {
        self.0.x() + Cast::<i32>::cast(self.1.width())
    }

    pub fn top(&self) -> i32 {
        self.0.y()
    }

    pub fn bottom(&self) -> i32 {
        self.0.y() + Cast::<i32>::cast(self.1.height())
    }

    pub fn width(&self) -> usize {
        self.1.width()
    }

    pub fn height(&self) -> usize {
        self.1.height()
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }

    pub fn iter(&self) -> IntExtent2DIterator {
        IntExtent2DIterator::new(self)
    }
}

pub struct IntExtent2DIterator<'a> {
    extent: &'a IntExtent2D,
    current: IntVector2,
}

impl<'a> IntExtent2DIterator<'a> {
    pub fn new(extent: &'a IntExtent2D) -> Self {
        Self {
            extent,
            current: extent.0,
        }
    }
}

impl<'a> Iterator for IntExtent2DIterator<'a> {
    type Item = IntVector2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y() >= self.extent.bottom() {
            return None;
        }

        let result = self.current;

        self.current = IntVector2::new(self.current.x() + 1, self.current.y());

        if self.current.x() >= self.extent.right() {
            self.current = IntVector2::new(self.extent.left(), self.current.y() + 1);
        }

        Some(result)
    }
}
