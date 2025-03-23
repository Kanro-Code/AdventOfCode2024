pub mod template;

// Use this file to add helper functions and additional modules.

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<T>>,
}

pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
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
            Direction::Northeast => (1, -1),
            Direction::East => (1, 0),
            Direction::Southeast => (1, 1),
            Direction::South => (0, 1),
            Direction::Southwest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::Northwest => (-1, -1),
        }
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn new(input: Vec<Vec<T>>) -> Grid<T> {
        Self {
            width: input[0].len(),
            height: input.len(),
            cells: input,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.cells.get(y).and_then(|row| row.get(x))
    }

    #[allow(clippy::result_unit_err)]
    pub fn collect_sequence(
        &self,
        x: usize,
        y: usize,
        distance: usize,
        direction: Direction,
    ) -> Result<Vec<&T>, ()> {
        let (dx, dy) = direction.delta_coords();

        (0..distance).try_fold(Vec::with_capacity(distance), |mut acc, i| {
            let new_x = (x as isize + dx * i as isize) as usize;
            let new_y = (y as isize + dy * i as isize) as usize;

            if new_x >= self.width || new_y >= self.height {
                return Err(());
            }

            match self.get(new_x, new_y) {
                Some(value) => {
                    acc.push(value);
                    Ok(acc)
                }
                None => Err(()),
            }
        })
    }

    #[allow(clippy::result_unit_err)]
    pub fn matches_grid(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
        expected: Vec<&T>,
    ) -> Result<bool, ()> {
        let seq = self.collect_sequence(x, y, expected.len(), direction)?;
        Ok(seq == expected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = super::Grid::new(input);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);

        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(1, 1), Some(&5));
        assert_eq!(grid.get(2, 2), Some(&9));
    }

    #[test]
    fn grid_get_direction() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = super::Grid::new(input);

        let vec = grid.collect_sequence(0, 0, 3, Direction::East);
        assert_eq!(vec.unwrap(), vec![&1, &2, &3]);

        let vec = grid.collect_sequence(0, 0, 3, Direction::South);
        assert_eq!(vec.unwrap(), vec![&1, &4, &7]);

        let vec = grid.collect_sequence(0, 2, 3, Direction::North);
        assert_eq!(vec.unwrap(), vec![&7, &4, &1]);

        let vec = grid.collect_sequence(2, 2, 3, Direction::West);
        assert_eq!(vec.unwrap(), vec![&9, &8, &7]);

        let vec = grid.collect_sequence(2, 2, 3, Direction::Northwest);
        assert_eq!(vec.unwrap(), vec![&9, &5, &1]);

        let vec = grid.collect_sequence(0, 2, 3, Direction::Northeast);
        assert_eq!(vec.unwrap(), vec![&7, &5, &3]);

        let vec = grid.collect_sequence(0, 0, 3, Direction::Southeast);
        assert_eq!(vec.unwrap(), vec![&1, &5, &9]);

        let vec = grid.collect_sequence(2, 0, 3, Direction::Southwest);
        assert_eq!(vec.unwrap(), vec![&3, &5, &7]);
    }

    #[test]
    fn grid_fails() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = super::Grid::new(input);

        let vec = grid.collect_sequence(0, 0, 4, Direction::East);
        assert!(vec.is_err());

        let vec = grid.collect_sequence(0, 0, 4, Direction::South);
        assert!(vec.is_err());
    }

    #[test]
    fn grid_matches_grid() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = super::Grid::new(input);

        assert!(grid.matches_grid(0, 0, Direction::East, vec![&1, &2, &3]).unwrap());
        assert!(!grid.matches_grid(0, 0, Direction::East, vec![&1, &2, &4]).unwrap());
    }
}
