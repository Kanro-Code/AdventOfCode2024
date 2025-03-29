use crate::{Direction, Grid, Point};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// GridIterator
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    current: Option<Point>,
    direction: Option<Direction>,
}

impl<'a, T> GridIterator<'a, T>
where
    T: PartialEq + Copy + Debug,
{
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            current: Some(Point { x: 0, y: 0 }),
            direction: None,
        }
    }

    pub fn calculate_next_point(&mut self) {
        let new = self.current.take().unwrap();

        if self.direction.is_none() {
            self.simple_walk(new);
        } else {
            self.complex_direction(new);
        }
    }

    fn simple_walk(&mut self, mut new: Point) {
        new.x += 1;
        if new.x >= self.grid.width {
            new.x = 0;
            new.y += 1;
        }
        if new.y >= self.grid.height {
            self.current = None;
        } else {
            self.current = Some(new);
        }
    }

    fn complex_direction(&mut self, mut new: Point) {
        let (dx, dy) = self.direction.as_ref().unwrap().delta();
        new.x += dx;
        new.y += dy;

        if self.grid.out_of_bounds(&new) {
            self.current = None;
        } else {
            self.current = Some(new);
        }
    }

    pub fn with_points(mut self) -> impl Iterator<Item = (Point, T)> + 'a
    where
        T: Clone,
    {
        std::iter::from_fn(move || {
            let point = self.current;

            self.next().map(|value| {
                (point.unwrap(), value)
            })
        })
    }

    pub fn custom(mut self, direction: Direction, point: Point) -> Self {
        self.direction = Some(direction);
        self.current = Some(point);
        self
    }
}

impl<T> Iterator for GridIterator<'_, T>
where
    T: Copy + PartialEq + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = &self.current {
            let value = self.grid.get(current);
            self.calculate_next_point();
            Some(value)
        } else {
            None
        }
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
    pub fn test_new_grid_iterator() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let mut iter = grid.iter();
        assert_eq!(iter.current, Some(Point { x: 0, y: 0 }));

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.current, Some(Point { x: 1, y: 0 }));

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(9));

    }

    #[test]
    pub fn test_grid_iterator_with_points() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let mut iter = grid.iter().with_points();
        assert_eq!(iter.next(), Some((Point { x: 0, y: 0 }, 1)));
        assert_eq!(iter.next(), Some((Point { x: 1, y: 0 }, 2)));
        assert_eq!(iter.next(), Some((Point { x: 2, y: 0 }, 3)));
        assert_eq!(iter.next(), Some((Point { x: 0, y: 1 }, 4)));
        assert_eq!(iter.next(), Some((Point { x: 1, y: 1 }, 5)));
        assert_eq!(iter.next(), Some((Point { x: 2, y: 1 }, 6)));
        assert_eq!(iter.next(), Some((Point { x: 0, y: 2 }, 7)));
        assert_eq!(iter.next(), Some((Point { x: 1, y: 2 }, 8)));
        assert_eq!(iter.next(), Some((Point { x: 2, y: 2 }, 9)));
    }

    #[test]
    pub fn test_grid_iter_with_direction() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let mut iter = grid
            .iter()
            .custom(Direction::SouthEast, Point { x: 0, y: 0 });
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.current, Some(Point { x: 1, y: 1 }));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.current, Some(Point { x: 2, y: 2 }));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.current, None);
    }
}
