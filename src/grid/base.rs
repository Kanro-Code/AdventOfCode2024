use crate::{Direction, GridIterator, Point};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    cells: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub fn new(input: Vec<Vec<T>>) -> Grid<T> {
        Self {
            width: input[0].len() as isize,
            height: input.len() as isize,
            cells: input,
        }
    }

    pub fn get(&self, point: &Point) -> Option<T> {
        self.cells
            .get(point.y as usize)
            .and_then(|row| row.get(point.x as usize).cloned())
    }

    pub fn matches(&self, point: &Point, direction: &Direction, expected: &[T]) -> bool {
        let mut count = 0;
        let res = self.iter()
            .with_route(point.clone(), direction.clone())
            .zip(expected.iter())
            .enumerate()
            .all(|(i, ((_, a), b))| {
                count = i;
                a == *b
            });

        if count != expected.len() - 1 || !res {
            return false;
        }

        true
    }

    pub fn get_values(&self, start: &Point, direction: &Direction, distance: isize) -> Vec<T> {
        unimplemented!()
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator::new(self)
    }

    pub fn out_of_bounds(&self, point: &Point, direction: &Direction, distance: isize) -> bool {
        let (dx, dy) = direction.delta();
        let new_x = point.x + dx * distance;
        let new_y = point.y + dy * distance;

        if new_x < 0 || new_x >= self.width {
            return true;
        }

        if new_y < 0 || new_y >= self.height {
            return true;
        }
        false
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
        let direction = Direction::East;
        let distance = [(0, false), (1, false), (2, false), (3, true), (4, true)];
        for (d, expected) in distance {
            assert_eq!(grid.out_of_bounds(&point, &direction, d), expected);
        }

        let point = Point { x: 2, y: 2 };
        let direction = Direction::NorthEast;
        let distance = 1;
        assert!(grid.out_of_bounds(&point, &direction, distance));

        let point = Point { x: 2, y: 2 };
        let direction = Direction::SouthWest;
        let distance = 1;
        assert!(grid.out_of_bounds(&point, &direction, distance));

        let point = Point { x: 2, y: 2 };
        let direction = Direction::North;
        let distance = 4;
        assert!(grid.out_of_bounds(&point, &direction, distance));
    }

    #[test]
    pub fn test_matches() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let point = Point { x: 0, y: 0 };
        let direction = Direction::East;
        let expected = vec![1, 2, 3];
        assert!(grid.matches(&point, &direction, &expected));

        let point = Point { x: 0, y: 0 };
        let direction = Direction::South;
        let expected = vec![1, 4, 7];
        assert!(grid.matches(&point, &direction, &expected));

        let point = Point { x: 0, y: 0 };
        let direction = Direction::SouthEast;
        let expected = vec![1, 5, 9];
        assert!(grid.matches(&point, &direction, &expected));

        let point = Point { x: 0, y: 0 };
        let direction = Direction::NorthWest;
        let expected = vec![1, 2, 3];
        assert!(!grid.matches(&point, &direction, &expected));

        let point = Point { x: 0, y: 0 };
        let direction = Direction::SouthWest;
        let expected = vec![1, 4, 7];
        assert!(!grid.matches(&point, &direction, &expected));
    }
}
