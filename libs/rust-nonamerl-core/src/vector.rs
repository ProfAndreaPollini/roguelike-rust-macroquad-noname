use crate::{Cast, Scalar};

pub trait Vector: 'static + Sized + Copy {
    type Scalar: Scalar;
}

pub trait Vec2: Vector + From<[Self::Scalar; 2]> {
    fn x(&self) -> Self::Scalar;
    fn y(&self) -> Self::Scalar;
    fn x_mut(&mut self) -> &mut Self::Scalar;
    fn y_mut(&mut self) -> &mut Self::Scalar;

    fn as_array<T>(self) -> [T; 2]
    where
        Self::Scalar: Cast<T>,
    {
        [self.x().cast(), self.y().cast()]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vector2<T: Scalar> {
    x: T,
    y: T,
}

impl<T: Scalar> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Scalar> Vec2 for Vector2<T> {
    fn x(&self) -> Self::Scalar {
        self.x
    }

    fn y(&self) -> Self::Scalar {
        self.y
    }

    fn x_mut(&mut self) -> &mut Self::Scalar {
        &mut self.x
    }

    fn y_mut(&mut self) -> &mut Self::Scalar {
        &mut self.y
    }

    fn as_array<T1>(self) -> [T1; 2]
    where
        Self::Scalar: Cast<T1>,
    {
        [self.x().cast(), self.y().cast()]
    }
}

impl<T: Scalar> Vector for Vector2<T> {
    type Scalar = T;
}

impl<T: Scalar> From<[T; 2]> for Vector2<T> {
    fn from(array: [T; 2]) -> Self {
        Self::new(array[0], array[1])
    }
}

pub type IntVector2 = Vector2<i32>;
