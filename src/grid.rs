use crate::{Point, Direction};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    cells: Vec<Vec<T>>,
}

impl <T> Grid<T> {
    pub fn new(input: Vec<Vec<T>>) -> Grid<T> {
        Self {
            width: input[0].len() as isize,
            height: input.len() as isize,
            cells: input,
        }
    }

    pub fn matches(&self, point: &Point, direction: &Direction, expected: &Vec<T>) -> bool {
        unimplemented!()
    }

    pub fn get_values(&self, start: &Point, direction: &Direction, distance: isize) -> Vec<T> {
        unimplemented!()
    }

    pub fn iter(&self) -> GridIterator<T> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    point: Point,
    next: Option<Point>,
}

impl <'a, T> GridIterator<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        let mut iter = Self {
            grid,
            point: Point { x: 0, y: 0 },
            next: None,
        };

        iter.next = iter.calculate_next_point();
        iter
    }

    pub fn current_point(&self) -> &Point {
        &self.point
    }

    pub fn next_point(&self) -> &Option<Point> {
        &self.next
    }

    pub fn calculate_next_point(&self) -> Option<Point> {
        let mut next = self.point.clone();

        next.x += 1;

        if next.x == self.grid.width {
            next.x = 0;
            next.y += 1;
        }

        if next.y == self.grid.height {
            None
        } else {
            Some(next)
        }
    }
}

impl <T> Iterator for GridIterator<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
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
}


// use crate::{Coordinate, Direction, GridIterator, GridDirectionIterator};

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub struct Grid<T> {
//     pub width: usize,
//     pub height: usize,
//     cells: Vec<Vec<T>>,
// }

// impl<T> Grid<T>
// where
//     T: Clone + PartialEq,
// {
//     pub fn new(input: Vec<Vec<T>>) -> Grid<T> {
//         Self {
//             width: input[0].len(),
//             height: input.len(),
//             cells: input,
//         }
//     }

//     pub fn get_value(&self, point: &Coordinate) -> Option<T> {
//         self.cells
//             .get(point.y)
//             .and_then(|row| row.get(point.x).cloned())
//     }

//     #[allow(clippy::result_unit_err)]
//     pub fn collect_sequence(
//         &self,
//         point: &Coordinate,
//         distance: usize,
//         direction: &Direction,
//     ) -> Result<Vec<T>, ()> {
//         let (dx, dy) = direction.delta();
//         let mut point = point.clone();

//         (0..distance).try_fold(Vec::with_capacity(distance), |mut acc, i| {
//             println!("{point:?}");

//             if point.x >= self.width || point.y >= self.height {
//                 return Err(());
//             }

//             point = point.offset_by(dx, dy);

//             match self.get_value(&point) {
//                 Some(value) => {
//                     acc.push(value);
//                     Ok(acc)
//                 }
//                 None => Err(()),
//             }

//         })
//     }

//     #[allow(clippy::result_unit_err)]
//     pub fn matches_sequence(
//         &self,
//         point: &Coordinate,
//         direction: &Direction,
//         expected: &Vec<T>,
//     ) -> Result<bool, ()> {
//         let seq = self.collect_sequence(point, expected.len(), direction)?;
//         Ok(&seq == expected)
//     }

//     pub fn iter(&self) -> GridIterator<T> {
//         GridIterator::new(self)
//     }

//     pub fn iter_direction(&self, start: Coordinate, direction: Direction) -> GridDirectionIterator<T> {
//         GridDirectionIterator::new(self, start, direction)
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn grid() {
//         let input = vec![
//             vec!['1', '2', '3'],
//             vec!['4', '5', '6'],
//             vec!['7', '8', '9'],
//         ];
//         let grid = super::Grid::new(input);
//         assert_eq!(grid.width, 3);
//         assert_eq!(grid.height, 3);

//         assert_eq!(grid.get_value(&Coordinate { x: 0, y: 0 }), Some('1'));
//         assert_eq!(grid.get_value(&Coordinate { x: 1, y: 1 }), Some('5'));
//         assert_eq!(grid.get_value(&Coordinate { x: 2, y: 2 }), Some('9'));
//     }

//     #[test]
//     fn grid_get_direction() {
//         let input = vec![
//             vec!['1', '2', '3'],
//             vec!['4', '5', '6'],
//             vec!['7', '8', '9'],
//         ];
//         let grid = super::Grid::new(input);

//         let vec = grid.collect_sequence(&Coordinate { x: 0, y: 0 }, 3, &Direction::East);
//         assert_eq!(vec.unwrap(), vec!['1', '2', '3']);

//         let vec = grid.collect_sequence(&Coordinate { x: 0, y: 0 }, 3, &Direction::South);
//         assert_eq!(vec.unwrap(), vec!['1', '4', '7']);

//         let vec = grid.collect_sequence(&Coordinate { x: 0, y: 2 }, 3, &Direction::North);
//         assert_eq!(vec.unwrap(), vec!['7', '4', '1']);

//         let vec = grid.collect_sequence(&Coordinate { x: 2, y: 2 }, 3, &Direction::West);
//         assert_eq!(vec.unwrap(), vec!['9', '8', '7']);

//         let vec = grid.collect_sequence(&Coordinate { x: 2, y: 2 }, 3, &Direction::NorthWest);
//         assert_eq!(vec.unwrap(), vec!['9', '5', '1']);

//         let vec = grid.collect_sequence(&Coordinate { x: 0, y: 2 }, 3, &Direction::NorthEast);
//         assert_eq!(vec.unwrap(), vec!['7', '5', '3']);

//         let vec = grid.collect_sequence(&Coordinate { x: 0, y: 0 }, 3, &Direction::SouthEast);
//         assert_eq!(vec.unwrap(), vec!['1', '5', '9']);

//         let vec = grid.collect_sequence(&Coordinate { x: 2, y: 0 }, 3, &Direction::SouthWest);
//         assert_eq!(vec.unwrap(), vec!['3', '5', '7']);
//     }

//     #[test]
//     fn grid_fails() {
//         let input = vec![
//             vec!['1', '2', '3'],
//             vec!['4', '5', '6'],
//             vec!['7', '8', '9'],
//         ];
//         let grid = super::Grid::new(input);

//         let vec = grid.collect_sequence(&Coordinate { x: 0, y: 0 }, 4, &Direction::East);
//         assert!(vec.is_err());

//         let vec = grid.collect_sequence(&Coordinate { x: 0, y: 0 }, 4, &Direction::South);
//         assert!(vec.is_err());
//     }

//     #[test]
//     fn grid_matches_grid() {
//         let input = vec![
//             vec!['1', '2', '3'],
//             vec!['4', '5', '6'],
//             vec!['7', '8', '9'],
//         ];
//         let grid = super::Grid::new(input);

//         assert!(grid
//             .matches_sequence(
//                 &Coordinate { x: 0, y: 0 },
//                 &Direction::East,
//                 &vec!['1', '2', '3']
//             )
//             .unwrap());
//         assert!(!grid
//             .matches_sequence(
//                 &Coordinate { x: 0, y: 0 },
//                 &Direction::East,
//                 &vec!['1', '2', '4']
//             )
//             .unwrap());
//     }
// }
