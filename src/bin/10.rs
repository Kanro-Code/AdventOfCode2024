use advent_of_code::{Direction, Grid, Point};
use std::collections::HashMap;

advent_of_code::solution!(10);

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let total: usize = input.iter().with_points().filter_map(|(point, v)| {
        if v == 0 {
            let count = walk_path(&input, point);
            return Some(count);
        }
        None
    }).sum();
    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let total: usize = input.iter().with_points().filter_map(|(point, v)| {
        if v == 0 {
            let count = walk_path(&input, point);
            return Some(count);
        }
        None
    }).sum();
    Some(total as u64)
}

pub fn walk_path(grid: &Grid<u32>, start: Point) -> usize {
    let mut found = HashMap::new();
    let current = grid.get(start);

    walk_path_recur(grid, start, current, &mut found);

    found.values().sum()
}

pub fn walk_path_recur(grid: &Grid<u32>, point: Point, current: u32, found: &mut HashMap<Point, usize>) {
    if current == 9 {
        *found.entry(point).or_insert(0) += 1
    }

    DIRECTIONS.iter().for_each(|direction| {
        let next = point.translate_direction(*direction);
        if grid.get_safe(next) == Some(current + 1) {
            walk_path_recur(grid, next, current + 1, found);
        }
    });
}

pub fn parse_input(input: &str) -> Grid<u32> {
    let vec: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars())
        .map(|chars| chars.map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Grid::new(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
