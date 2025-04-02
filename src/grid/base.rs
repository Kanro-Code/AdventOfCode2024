use crate::{Direction, GridIterator, Point};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    cells: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Copy + PartialEq + Debug + Default,
{
    pub fn new(input: Vec<Vec<T>>) -> Grid<T> {
        Self {
            width: input[0].len() as isize,
            height: input.len() as isize,
            cells: input,
        }
    }

    pub fn new_empty(width: isize, height: isize) -> Grid<T> {
        let mut cells = Vec::with_capacity(height as usize);
        for _ in 0..height {
            cells.push(vec![T::default(); width as usize]);
        }
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, point: Point) -> T {
        self.cells[point.y as usize][point.x as usize]
    }

    pub fn set(&mut self, point: Point, value: T) {
        self.cells[point.y as usize][point.x as usize] = value;
    }

    pub fn set_safe(&mut self, point: Point, value: T) -> bool {
        if self.out_of_bounds(&point) {
            return false;
        }

        self.set(point, value);
        true
    }

    pub fn matches(&self, point: Point, direction: Direction, expected: &[T]) -> bool {
        let mut iter = self
            .iter()
            .in_direction(direction, point)
            .take(expected.len());

        expected.iter().all(|e| match iter.next() {
            Some(value) => value == *e,
            None => false,
        })
    }

    pub fn get_values(&self, start: Point, direction: Direction, distance: usize) -> Vec<T> {
        let values: Vec<T> = self
            .iter()
            .in_direction(direction, start)
            .take(distance)
            .collect();

        if values.len() != distance {
            panic!("Out of bounds");
        }
        values
    }

    pub fn out_of_bounds(&self, point: &Point) -> bool {
        if point.x < 0 || point.x >= self.width {
            return true;
        }

        if point.y < 0 || point.y >= self.height {
            return true;
        }
        false
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new_grid() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
    }

    #[test]
    pub fn test_out_of_bounds() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let point = Point { x: 0, y: 0 };
        assert!(!grid.out_of_bounds(&point));

        let point = Point { x: 1, y: 1 };
        assert!(!grid.out_of_bounds(&point));

        let point = Point { x: 2, y: 2 };
        assert!(!grid.out_of_bounds(&point));

        let point = Point { x: 3, y: 3 };
        assert!(grid.out_of_bounds(&point));

        let point = Point { x: -1, y: -1 };
        assert!(grid.out_of_bounds(&point));

        let point = Point { x: 3, y: 0 };
        assert!(grid.out_of_bounds(&point));

        let point = Point { x: 0, y: 3 };
        assert!(grid.out_of_bounds(&point));

        let point = Point { x: -1, y: 0 };
        assert!(grid.out_of_bounds(&point));

        let point = Point { x: 0, y: -1 };
        assert!(grid.out_of_bounds(&point));
    }

    #[test]
    pub fn test_matches() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let point = Point { x: 0, y: 0 };
        let direction = Direction::East;
        let expected = vec![1, 2, 3];
        assert!(grid.matches(point, direction, &expected));

        let point = Point { x: 0, y: 0 };
        let direction = Direction::South;
        let expected = vec![1, 4, 7];
        assert!(grid.matches(point, direction, &expected));

        let point = Point { x: 0, y: 0 };
        let direction = Direction::SouthEast;
        let expected = vec![1, 5, 9];
        assert!(grid.matches(point, direction, &expected));

        let point = Point { x: 0, y: 0 };
        let direction = Direction::NorthWest;
        let expected = vec![1, 2, 3];
        assert!(!grid.matches(point, direction, &expected));

        let point = Point { x: 0, y: 0 };
        let direction = Direction::SouthWest;
        let expected = vec![1, 4, 7];
        assert!(!grid.matches(point, direction, &expected));

        let point = Point { x: 2, y: 2 };
        let direction = Direction::North;
        let expected = vec![9];
        assert!(grid.matches(point, direction, &expected));

        let point = Point { x: 2, y: 2 };
        let direction = Direction::South;
        let expected = vec![9];
        assert!(grid.matches(point, direction, &expected));
    }

    #[test]
    pub fn test_get_values() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let point = Point { x: 0, y: 0 };
        let direction = Direction::East;
        let expected = vec![1, 2, 3];
        assert_eq!(grid.get_values(point, direction, 3), expected);

        let point = Point { x: 0, y: 0 };
        let direction = Direction::South;
        let expected = vec![1, 4, 7];

        assert_eq!(grid.get_values(point, direction, 3), expected);
        let point = Point { x: 0, y: 0 };
        let direction = Direction::SouthEast;
        let expected = vec![1, 5, 9];
        assert_eq!(grid.get_values(point, direction, 3), expected);
    }

    #[test]
    #[should_panic]
    pub fn test_get_values_out_of_bounds() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let point = Point { x: 0, y: 0 };
        let direction = Direction::East;
        grid.get_values(point, direction, 4);
    }
}
