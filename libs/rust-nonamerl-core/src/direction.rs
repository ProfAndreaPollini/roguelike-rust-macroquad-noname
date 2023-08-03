use crate::vector::Vector2;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Vector2<i32> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Vector2::new(0, -1),
            Direction::Down => Vector2::new(0, 1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::vector::Vec2;
    use crate::vector::Vector2;

    #[test]
    fn direction_enum_works() {
        let up = Direction::Up;
        let v = Vector2::from(up);

        assert_eq!(v.x(), 0);
        assert_eq!(v.y(), -1);
    }
}
