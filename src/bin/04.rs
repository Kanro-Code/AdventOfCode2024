advent_of_code::solution!(4);

use std::vec;

use advent_of_code::{Direction, Grid};

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let xmas = vec![&'X', &'M', &'A', &'S'];

    let directions = [
        Direction::South,
        Direction::East,
        Direction::SouthWest,
        Direction::SouthEast,
        Direction::West,
        Direction::NorthWest,
        Direction::North,
        Direction::NorthEast,
    ];

    let total = grid
        .iter()
        .map(|(x, y, _)| {
            directions.iter().fold(0, |acc, dir| {
                let res = grid.matches_grid(x, y, dir, &xmas);
                if let Ok(res) = res {
                    if res {
                        return acc + 1;
                    }
                }
                acc
            })
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let check_sequence = |x, y, dir| {
        grid.collect_sequence(x, y, 3, dir)
            .map(|seq| seq.iter().map(|c| **c as u64).sum::<u64>())
            .unwrap_or(0)
    };

    let total = (1..grid.width - 1)
        .flat_map(|x| (1..grid.height - 1).map(move |y| (x, y)))
        .filter(|&(x, y)| grid.get_cell(x, y) == Some(&'A'))
        .filter(|&(x, y)| {
            let part1 = check_sequence(x - 1, y - 1, &Direction::SouthEast);
            let part2 = check_sequence(x + 1, y - 1, &Direction::SouthWest);
            part1 == 225 && part2 == 225
        })
        .count();

    Some(total as u64)
}

pub fn parse_input(input: &str) -> Grid<char> {
    let vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::new(vec)
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
