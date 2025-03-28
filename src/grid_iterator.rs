use crate::{Grid, Point, Direction};

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

impl <'a, T> GridIterator<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            current: Some(Point { x: 0, y: 0 }),
            direction: None,
        }
    }

    pub fn calculate_next_point(&self) -> Option<Point> {
        self.current.as_ref().and_then(|current_point| {
            let mut point = current_point.clone();
            point.x += 1;

            if point.x >= self.grid.width {
                point.x = 0;
                point.y += 1;
            }

            if point.y >= self.grid.height {
                None
            } else {
                Some(point)
            }
        })
    }

    pub fn with_points(mut self) -> impl Iterator<Item = (Option<Point>, T)> + 'a
    where
        T: Clone,
    {
        std::iter::from_fn(move || {
            let point = self.current.clone();
            self.next().map(|value| {
                (point, value)
            })
        })
    }
}

impl <T> Iterator for GridIterator<'_, T> where T: Clone {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = &self.current {
            let value = self.grid.get(current);
            self.current = self.calculate_next_point();
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
}

// use crate::{Coordinate, Direction, Grid};

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub struct Base<'a, T> {
//     grid: &'a Grid<T>,
//     point: Coordinate,
// }

// impl <'a, T> Base <'a, T> {
//     pub fn new(grid: &'a Grid<T>) -> Self {
//         Self {
//             grid,
//             point: Coordinate { x: 0, y: 0 },
//         }
//     }

//     pub fn new_with_point(grid: &'a Grid<T>, point: Coordinate) -> Self {
//         Self { grid, point }
//     }
// }

// impl<T> Iterator for Base<'_, T>
// where
//     T: Clone + PartialEq,
// {
//     type Item = (Coordinate, T);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.point.y == self.grid.height {
//             return None;
//         }
//         let value = self.grid.get_value(&self.point).unwrap();
//         let point = Coordinate {
//             x: self.point.x,
//             y: self.point.y,
//         };

//         self.point.x += 1;
//         if self.point.x == self.grid.width {
//             self.point.x = 0;
//             self.point.y += 1;
//         }

//         Some((point, value))
//     }
// }

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub struct Travel<'a, T> {
//     iter: Base<'a, T>,
//     direction: Direction,
// }

// impl <'a, T> Travel <'a, T> {
//     pub fn new(grid: &'a Grid<T>, start: Coordinate, direction: Direction) -> Self {
//         Self {
//             iter: Base::new_with_point(grid, start),
//             direction,
//         }
//     }
// }

// impl <T> Iterator for Travel<'_, T>
// where
//     T: Clone + PartialEq,
// {
//     type Item = (Coordinate, T);

//     fn next(&mut self) -> Option<Self::Item> {
//         let next_point = self.direction.offset_by(&self.iter.point);
//         let current_pos = std::mem::replace(&mut self.iter.point, next_point);

//         let value = self.iter.grid.get_value(&current_pos).unwrap();
//         Some((current_pos, value))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn grid_iterator() {
//         let input = vec![vec!['1', '2'], vec!['3', '4']];
//         let grid = super::Grid::new(input);

//         let mut iter = grid.iter();
//         assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 0 }, '1')));
//         assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 0 }, '2')));
//         assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 1 }, '3')));
//         assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 1 }, '4')));
//         assert_eq!(iter.next(), None);
//     }

//     #[test]
//     fn grid_iterator_direction() {
//         let input = vec![vec![true, true, false, true, false], vec![true, true, true, false, false]];
//         let grid = super::Grid::new(input);

//         let mut iter = grid.iter_direction(Coordinate { x: 0, y: 0 }, Direction::East);
//         assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 0 }, true)));
//         assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 0 }, true)));
//         assert_eq!(iter.next(), Some((Coordinate { x: 2, y: 0 }, false)));
//         assert_eq!(iter.next(), Some((Coordinate { x: 3, y: 0 }, true)));
//         assert_eq!(iter.next(), Some((Coordinate { x: 4, y: 0 }, false)));
//         assert_eq!(iter.next(), None);

//         let mut iter = grid.iter_direction(Coordinate { x: 5, y: 1 }, Direction::East);
//         assert_eq!(iter.next(), Some((Coordinate { x: 4, y: 1 }, false)));
//         assert_eq!(iter.next(), Some((Coordinate { x: 3, y: 1 }, false)));
//         assert_eq!(iter.next(), Some((Coordinate { x: 2, y: 1 }, true)));
//         assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 1 }, true)));
//         assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 1 }, true)));
//         assert_eq!(iter.next(), None);
//     }
// }
