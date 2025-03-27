#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<T>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    /// Returns the vector of the direction
    /// The vector is a tuple of (dx, dy)
    ///
    /// - Negative dx moves west
    /// - Negative dy moves north
    /// - Positive dx moves east
    /// - Positive dy moves south
    pub fn delta_coords(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }

    pub fn out_of_bounds(&self, coor: &Coordinate, width: usize, height: usize) -> bool {
        let (dx, dy) = self.delta_coords();

        if coor.x as isize + dx < 0 || coor.x as isize + dx >= width as isize {
            return true;
        }

        if coor.y as isize + dy < 0 || coor.y as isize + dy > height as isize {
            return true;
        }

        false
    }
}

impl<T> Grid<T>
where
    T: Clone + PartialEq,
{
    pub fn new(input: Vec<Vec<T>>) -> Grid<T> {
        Self {
            width: input[0].len(),
            height: input.len(),
            cells: input,
        }
    }

    pub fn get_cell(&self, coor: &Coordinate) -> Option<T> {
        if coor.x >= self.width || coor.y >= self.height {
            return None;
        }

        self.cells.get(coor.y).and_then(|row| row.get(coor.x).cloned())
    }

    #[allow(clippy::result_unit_err)]
    pub fn collect_sequence(
        &self,
        x: usize,
        y: usize,
        distance: usize,
        direction: &Direction,
    ) -> Result<Vec<T>, ()> {
        let (dx, dy) = direction.delta_coords();

        (0..distance).try_fold(Vec::with_capacity(distance), |mut acc, i| {
            let new_x = (x as isize + dx * i as isize) as usize;
            let new_y = (y as isize + dy * i as isize) as usize;

            if new_x >= self.width || new_y >= self.height {
                return Err(());
            }

            let coor = Coordinate { x: new_x, y: new_y };

            match self.get_cell(&coor) {
                Some(value) => {
                    acc.push(value);
                    Ok(acc)
                }
                None => Err(()),
            }
        })
    }

    #[allow(clippy::result_unit_err)]
    pub fn matches_sequence(
        &self,
        coor: &Coordinate,
        direction: &Direction,
        expected: &Vec<T>,
    ) -> Result<bool, ()> {
        let seq = self.collect_sequence(coor.x, coor.y, expected.len(), direction)?;
        Ok(&seq == expected)
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            coor: Coordinate { x: 0, y: 0 },
        }
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    coor: Coordinate,
}

impl<T> Iterator for GridIterator<'_, T>
where
    T: Clone + PartialEq,
{
    type Item = (Coordinate, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.coor.y == self.grid.height {
            return None;
        }
        let value = self.grid.get_cell(&self.coor).unwrap();
        let coor = Coordinate { x: self.coor.x, y: self.coor.y };

        self.coor.x += 1;
        if self.coor.x == self.grid.width {
            self.coor.x = 0;
            self.coor.y += 1;
        }

        Some((coor, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let grid = super::Grid::new(input);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);

        assert_eq!(grid.get_cell(&Coordinate { x: 0, y: 0 }), Some('1'));
        assert_eq!(grid.get_cell(&Coordinate { x: 1, y: 1 }), Some('5'));
        assert_eq!(grid.get_cell(&Coordinate { x: 2, y: 2 }), Some('9'));
    }

    #[test]
    fn grid_get_direction() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let grid = super::Grid::new(input);

        let vec = grid.collect_sequence(0, 0, 3, &Direction::East);
        assert_eq!(vec.unwrap(), vec!['1', '2', '3']);

        let vec = grid.collect_sequence(0, 0, 3, &Direction::South);
        assert_eq!(vec.unwrap(), vec!['1', '4', '7']);

        let vec = grid.collect_sequence(0, 2, 3, &Direction::North);
        assert_eq!(vec.unwrap(), vec!['7', '4', '1']);

        let vec = grid.collect_sequence(2, 2, 3, &Direction::West);
        assert_eq!(vec.unwrap(), vec!['9', '8', '7']);

        let vec = grid.collect_sequence(2, 2, 3, &Direction::NorthWest);
        assert_eq!(vec.unwrap(), vec!['9', '5', '1']);

        let vec = grid.collect_sequence(0, 2, 3, &Direction::NorthEast);
        assert_eq!(vec.unwrap(), vec!['7', '5', '3']);

        let vec = grid.collect_sequence(0, 0, 3, &Direction::SouthEast);
        assert_eq!(vec.unwrap(), vec!['1', '5', '9']);

        let vec = grid.collect_sequence(2, 0, 3, &Direction::SouthWest);
        assert_eq!(vec.unwrap(), vec!['3', '5', '7']);
    }

    #[test]
    fn grid_fails() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let grid = super::Grid::new(input);

        let vec = grid.collect_sequence(0, 0, 4, &Direction::East);
        assert!(vec.is_err());

        let vec = grid.collect_sequence(0, 0, 4, &Direction::South);
        assert!(vec.is_err());
    }

    #[test]
    fn grid_matches_grid() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let grid = super::Grid::new(input);

        assert!(grid
            .matches_sequence(&Coordinate { x: 0, y: 0 }, &Direction::East, &vec!['1', '2', '3'])
            .unwrap());
        assert!(!grid
            .matches_sequence(&Coordinate { x: 0, y: 0 }, &Direction::East, &vec!['1', '2', '4'])
            .unwrap());
    }

    #[test]
    fn grid_iterator() {
        let input = vec![vec!['1', '2'], vec!['3', '4']];
        let grid = super::Grid::new(input);

        let mut iter = grid.iter();
        assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 0 }, '1')));
        assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 0 }, '2')));
        assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 1 }, '3')));
        assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 1 }, '4')));
        assert_eq!(iter.next(), None);
    }
}
