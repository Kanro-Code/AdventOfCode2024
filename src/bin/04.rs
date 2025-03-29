advent_of_code::solution!(4);

use std::vec;

use advent_of_code::{Direction, Grid, Point};

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
        if value != 'X' {
            continue;
        }

        for direction in DIRECTIONS {
            if grid.matches(point, direction, &xmas) {
                total += 1;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let total = grid
        .iter()
        .with_points()
        .filter(|(point, value)| {
            if point.x == 0
                || point.x == grid.width - 1
                || point.y == 0
                || point.y == grid.height - 1
            {
                return false;
            }

            if value != &'A' {
                return false;
            }

            let values: u64 = get_corner_values(&grid, point).iter().sum();

            values == 320
        })
        .count();

    Some(total as u64)
}

pub fn parse_input(input: &str) -> Grid<char> {
    let vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::new(vec)
}

fn get_corner_values(grid: &Grid<char>, point: &Point) -> [u64; 4] {
    let corners = [
        Point {
            x: point.x - 1,
            y: point.y - 1,
        }, // Top left
        Point {
            x: point.x + 1,
            y: point.y - 1,
        }, // Top right
        Point {
            x: point.x - 1,
            y: point.y + 1,
        }, // Bottom left
        Point {
            x: point.x + 1,
            y: point.y + 1,
        }, // Bottom right
    ];

    corners.map(|p| grid.get(p) as u64)
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
