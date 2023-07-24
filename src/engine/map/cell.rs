use zorder::index_of;

use super::bresenham;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Cell {
    pub x: u16,
    pub y: u16,
}

impl Cell {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn id(&self) -> u32 {
        index_of((self.x, self.y))
    }

    pub fn distance_to(&self, target: &Cell) -> f32 {
        let x = (self.x as f32 - target.x as f32).abs();
        let y = (self.y as f32 - target.y as f32).abs();

        (x * x + y * y).sqrt()
    }

    pub fn neighbor_ids(&self) -> Vec<u32> {
        let mut neighbors = Vec::new();

        let x = self.x;
        let y = self.y;

        neighbors.push(index_of((x - 1, y - 1)));
        neighbors.push(index_of((x, y - 1)));
        neighbors.push(index_of((x + 1, y - 1)));
        neighbors.push(index_of((x - 1, y)));
        neighbors.push(index_of((x + 1, y)));
        neighbors.push(index_of((x - 1, y + 1)));
        neighbors.push(index_of((x, y + 1)));
        neighbors.push(index_of((x + 1, y + 1)));

        neighbors
    }

    pub fn line_to(&self, target: &Cell) -> Vec<u32> {
        bresenham::line(
            self.x as isize,
            self.y as isize,
            target.x as isize,
            target.y as isize,
        )
        .iter()
        .map(|p| index_of((p.0 as u16, p.1 as u16)))
        .collect()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_cell_id() {
        let cell = Cell::new(0, 0);
        assert_eq!(cell.id(), 0);

        let cell = Cell::new(1, 0);
        assert_eq!(cell.id(), 1);

        let cell = Cell::new(0, 1);
        assert_eq!(cell.id(), 2);

        let cell = Cell::new(1, 1);
        assert_eq!(cell.id(), 3);
    }

    #[test]
    fn test_cell_line_to() {
        let cell = Cell::new(0, 0);
        let target = Cell::new(1, 1);
        assert_eq!(cell.line_to(&target), vec![0, 3]);

        let cell = Cell::new(1, 1);
        let target = Cell::new(0, 0);
        assert_eq!(cell.line_to(&target), vec![3, 0]);
    }
}
