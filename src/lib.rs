pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<char>>,
}

#[derive(Debug)]
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

    pub fn out_of_bounds(&self, x: usize, y: usize, width: usize, height: usize) -> bool {
        let (dx, dy) = self.delta_coords();

        if x as isize + dx < 0 || x as isize + dx >= width as isize {
            return true;
        }

        if y as isize + dy < 0 || y as isize + dy > height as isize {
            return true
        }

        false
    }
}

impl Grid {
    pub fn new(input: Vec<Vec<char>>) -> Grid {
        Self {
            width: input[0].len(),
            height: input.len(),
            cells: input,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.cells.get(y).and_then(|row| row.get(x).cloned())

    }

    #[allow(clippy::result_unit_err)]
    pub fn collect_sequence(
        &self,
        x: usize,
        y: usize,
        distance: usize,
        direction: &Direction,
    ) -> Result<Vec<char>, ()> {
        let (dx, dy) = direction.delta_coords();

        (0..distance).try_fold(Vec::with_capacity(distance), |mut acc, i| {
            let new_x = (x as isize + dx * i as isize) as usize;
            let new_y = (y as isize + dy * i as isize) as usize;

            if new_x >= self.width || new_y >= self.height {
                return Err(());
            }

            match self.get_cell(new_x, new_y) {
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
        x: usize,
        y: usize,
        direction: &Direction,
        expected: &Vec<char>,
    ) -> Result<bool, ()> {
        let seq = self.collect_sequence(x, y, expected.len(), direction)?;
        Ok(&seq == expected)
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl Iterator for GridIterator<'_> {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.grid.height {
            return None;
        }
        let value = self.grid.get_cell(self.x, self.y).unwrap();
        let x = self.x;
        let y = self.y;

        self.x += 1;
        if self.x == self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        Some((x, y, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        let input = vec![vec!['1', '2', '3'], vec!['4', '5', '6'], vec!['7', '8', '9']];
        let grid = super::Grid::new(input);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);

        assert_eq!(grid.get_cell(0, 0), Some('1'));
        assert_eq!(grid.get_cell(1, 1), Some('5'));
        assert_eq!(grid.get_cell(2, 2), Some('9'));
    }

    #[test]
    fn grid_get_direction() {
        let input = vec![vec!['1', '2', '3'], vec!['4', '5', '6'], vec!['7', '8', '9']];
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
        let input = vec![vec!['1', '2', '3'], vec!['4', '5', '6'], vec!['7', '8', '9']];
        let grid = super::Grid::new(input);

        let vec = grid.collect_sequence(0, 0, 4, &Direction::East);
        assert!(vec.is_err());

        let vec = grid.collect_sequence(0, 0, 4, &Direction::South);
        assert!(vec.is_err());
    }

    #[test]
    fn grid_matches_grid() {
        let input = vec![vec!['1', '2', '3'], vec!['4', '5', '6'], vec!['7', '8', '9']];
        let grid = super::Grid::new(input);

        assert!(grid
            .matches_sequence(0, 0, &Direction::East, &vec!['1', '2', '3'])
            .unwrap());
        assert!(!grid
            .matches_sequence(0, 0, &Direction::East, &vec!['1', '2', '4'])
            .unwrap());
    }

    #[test]
    fn grid_iterator() {
        let input = vec![vec!['1', '2'], vec!['3', '4']];
        let grid = super::Grid::new(input);

        let mut iter = grid.iter();
        assert_eq!(iter.next(), Some((0, 0, '1')));
        assert_eq!(iter.next(), Some((1, 0, '2')));
        assert_eq!(iter.next(), Some((0, 1, '3')));
        assert_eq!(iter.next(), Some((1, 1, '4')));
        assert_eq!(iter.next(), None);
    }
}
