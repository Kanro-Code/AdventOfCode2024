advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let twodee = Grid::new(input);
    Some(twodee.scan())
}

pub fn part_two(input: &str) -> Option<u64> {
    // let input = Grid::parse_input(input);
    let grid = Grid::new(input);

    let count_patterns = |y: usize, x: usize| -> bool {
        if grid.cells[y][x] != 'A' {
            return false;
        }

        let corners = [
            (y - 1, x - 1),
            (y - 1, x + 1),
            (y + 1, x - 1),
            (y + 1, x + 1),
        ];

        // Adding values of chars on top of each other from the different corners
        let sum: u64 = corners
            .iter()
            .filter_map(|(y, x)| grid.cells.get(*y).unwrap().get(*x).map(|c| *c as u64))
            .sum();

        // println!("{}", sum == 320);
        // println!("{}-{}", grid.cells[y - 1][x - 1], grid.cells[y - 1][x + 1]);
        // println!("-A-");
        // println!("{}-{}", grid.cells[y + 1][x - 1], grid.cells[y + 1][x + 1]);
        // println!("---------------");
        // M + M + S + S equals to 320
        sum == 320
    };

    let total = (1..grid.height - 1)
        .flat_map(|y| (1..grid.width - 1).filter(move |&x| count_patterns(y, x)))
        .count() as u64;

    Some(total)
}

pub struct Grid {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(input: &str) -> Grid {
        let cells = Self::parse_input(input);
        Grid {
            width: cells[0].len(),
            height: cells.len(),
            cells,
        }
    }

    pub fn scan(&self) -> u64 {
        let mut total = 0;
        total += self.horizontal_matches();
        total += self.vertical_matches();
        total += self.diagonal_forward_matches();
        total += self.diagonal_backward_matches();

        total
    }

    fn parse_input(input: &str) -> Vec<Vec<char>> {
        let lines = input.lines().collect::<Vec<_>>();
        lines.iter().map(|line| line.chars().collect()).collect()
    }

    pub fn horizontal_matches(&self) -> u64 {
        (0..self.height)
            .map(|y| {
                (0..=self.width.saturating_sub(4))
                    .filter(|x| self.is_xmas_at(*x, y, 1, 0))
                    .count() as u64
            })
            .sum()
    }

    pub fn vertical_matches(&self) -> u64 {
        (0..self.width)
            .map(|x| {
                (0..=self.height.saturating_sub(4))
                    .filter(|y| self.is_xmas_at(x, *y, 0, 1))
                    .count() as u64
            })
            .sum()
    }

    pub fn diagonal_forward_matches(&self) -> u64 {
        (0..=self.width.saturating_sub(4))
            .map(|x| {
                (0..=self.height.saturating_sub(4))
                    .filter(|y| self.is_xmas_at(x, *y, 1, 1))
                    .count() as u64
            })
            .sum()
    }

    pub fn diagonal_backward_matches(&self) -> u64 {
        (0..self.width)
            .map(|x| {
                (0..=self.height.saturating_sub(4))
                    .filter(|y| self.is_xmas_at(x, *y, -1, 1))
                    .count() as u64
            })
            .sum()
    }

    pub fn is_xmas_at(&self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        let chars: Vec<char> = (0..4)
            .filter_map(|i| {
                let new_x = (x as isize + dx * i) as usize;
                let new_y = (y as isize + dy * i) as usize;

                self.cells.get(new_y).unwrap().get(new_x).copied()
            })
            .collect();

        Self::is_xmas_sequence(&chars)
    }

    fn is_xmas_sequence(chars: &[char]) -> bool {
        matches!(chars, &['X', 'M', 'A', 'S'] | &['S', 'A', 'M', 'X'])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
