use crate::{Direction, Grid, Point};

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
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            current: Some(Point { x: 0, y: 0 }),
            direction: None,
        }
    }

    pub fn calculate_next(&mut self) {
        if self.direction.is_some() {
            self.calculate_next_in_route();
            return;
        }

        let mut current = self.current.take().unwrap();

        current.x += 1;
        if current.x >= self.grid.width {
            current.x = 0;
            current.y += 1;
        }
        if current.y >= self.grid.height {
            self.current = None;
        } else {
            self.current = Some(current);
        }
    }

    pub fn calculate_next_in_route(&mut self) {
        let mut current = self.current.take().unwrap();
        let direction = self.direction.as_ref().unwrap();
        let (dx, dy) = direction.delta();

        let new_x = current.x + dx;
        let new_y = current.y + dy;

        if self.grid.out_of_bounds(&current, direction, 1) {
            self.current = None;
            return;
        }

        current.x = new_x;
        current.y = new_y;

        self.current = Some(current);
    }

    pub fn with_points(mut self) -> impl Iterator<Item = (Option<Point>, T)> + 'a
    where
        T: Clone,
    {
        std::iter::from_fn(move || {
            let point = self.current.clone();
            self.next().map(|value| (point, value))
        })
    }

    pub fn with_route(
        mut self,
        start: Point,
        direction: Direction,
    ) -> impl Iterator<Item = (Option<Point>, T)> + 'a
    where
        T: Clone + PartialEq,
    {
        self.current = Some(start);
        self.direction = Some(direction);
        std::iter::from_fn(move || {
            let point = self.current.clone();
            self.next().map(|value| (point, value))
        })
    }
}

impl<T> Iterator for GridIterator<'_, T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = &self.current {
            let value = self.grid.get(current);
            self.calculate_next();
            value
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

        let grid: Grid<bool> = Grid::new(vec![vec![]]);
        let iter = grid.iter();
        assert_eq!(iter.current, Some(Point { x: 0, y: 0 }));
        for (count, e) in iter.enumerate() {
            assert!(!e);
            assert_eq!(count, 0);
        }
    }

    #[test]
    pub fn test_grid_iterator_with_points() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let mut iter = grid.iter().with_points();
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 0 }), 1)));
        assert_eq!(iter.next(), Some((Some(Point { x: 1, y: 0 }), 2)));
        assert_eq!(iter.next(), Some((Some(Point { x: 2, y: 0 }), 3)));
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 1 }), 4)));
        assert_eq!(iter.next(), Some((Some(Point { x: 1, y: 1 }), 5)));
        assert_eq!(iter.next(), Some((Some(Point { x: 2, y: 1 }), 6)));
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 2 }), 7)));
        assert_eq!(iter.next(), Some((Some(Point { x: 1, y: 2 }), 8)));
        assert_eq!(iter.next(), Some((Some(Point { x: 2, y: 2 }), 9)));
    }

    #[test]
    pub fn test_grid_custom_crawl() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let mut iter = grid
            .iter()
            .with_route(Point { x: 0, y: 0 }, Direction::East);
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 0 }), 1)));
        assert_eq!(iter.next(), Some((Some(Point { x: 1, y: 0 }), 2)));
        assert_eq!(iter.next(), Some((Some(Point { x: 2, y: 0 }), 3)));
        assert_eq!(iter.next(), None);

        let mut iter = grid
            .iter()
            .with_route(Point { x: 0, y: 0 }, Direction::South);
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 0 }), 1)));
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 1 }), 4)));
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 2 }), 7)));

        let mut iter = grid
            .iter()
            .with_route(Point { x: 0, y: 0 }, Direction::North);
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 0 }), 1)));
        assert_eq!(iter.next(), None);

        let mut iter = grid
            .iter()
            .with_route(Point { x: 0, y: 0 }, Direction::SouthEast);
        assert_eq!(iter.next(), Some((Some(Point { x: 0, y: 0 }), 1)));
        assert_eq!(iter.next(), Some((Some(Point { x: 1, y: 1 }), 5)));
        assert_eq!(iter.next(), Some((Some(Point { x: 2, y: 2 }), 9)));
        assert_eq!(iter.next(), None);
    }
}
