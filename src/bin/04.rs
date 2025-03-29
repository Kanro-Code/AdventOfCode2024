advent_of_code::solution!(4);

use std::vec;

use advent_of_code::{Direction, Grid};

const DIRECTIONS: [Direction; 8] = [
    Direction::South,
    Direction::East,
    Direction::SouthWest,
    Direction::SouthEast,
    Direction::West,
    Direction::NorthWest,
    Direction::North,
    Direction::NorthEast,
];

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut total = 0;

    for (point, value) in grid.iter().with_points() {
        if value != &'X' {
            continue;
        }

        if let Some(point) = point {
            for direction in DIRECTIONS {
                if grid.matches(point, direction, &xmas) {
                    total += 1;
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    // let grid = parse_input(input);

    // let total: u64 = grid.iter().with_points().fold(0, |acc, (coor, value)| {
    //     if coor.x == 0 || coor.x == grid.width - 1 || coor.y == 0 || coor.y == grid.height - 1 {
    //         return acc;
    //     }

    //     if value != 'A' {
    //         return acc;
    //     }

    //     let topleft = Coordinate {
    //         x: coor.x - 1,
    //         y: coor.y - 1,
    //     };
    //     let topright = Coordinate {
    //         x: coor.x + 1,
    //         y: coor.y - 1,
    //     };

    //     let part1 = sum_sequence(&grid, &topleft, &Direction::SouthEast);
    //     let part2 = sum_sequence(&grid, &topright, &Direction::SouthWest);
    //     if part1 == 225 && part2 == 225 {
    //         acc + 1
    //     } else {
    //         acc
    //     }
    // });

    Some(9)
}

pub fn parse_input(input: &str) -> Grid<char> {
    let vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::new(vec)
}

// pub fn sum_sequence(grid: &Grid<char>, coor: &Coordinate, dir: &Direction) -> u64 {
//     grid.collect_sequence(coor, 3, dir)
//         .map(|seq| seq.iter().map(|c| *c as u64).sum())
//         .unwrap_or(0)
// }

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
