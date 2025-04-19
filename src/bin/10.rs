use advent_of_code::{Direction, Grid, Point};

advent_of_code::solution!(10);

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

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

pub fn walk_path(grid: &Grid<u32>, start: Point) -> usize {
    let mut found = Vec::new();
    let current = grid.get(start);

    walk_path_recur(grid, start, current, &mut found);

    found.len()
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

pub fn walk_path_recur(grid: &Grid<u32>, point: Point, current: u32, found: &mut Vec<Point>) {
    if current == 9 {
        if !found.contains(&point) {
            found.push(point);
        }
        return;
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
        assert_eq!(result, None);
    }
}
