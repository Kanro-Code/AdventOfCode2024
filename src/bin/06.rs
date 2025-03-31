use advent_of_code::{Direction, Grid, Point};
use std::collections::HashSet;

advent_of_code::solution!(6);

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub fn part_one(input: &str) -> Option<u64> {
    let (start, walls) = parse_input(input);
    let visited_cells = visited_route(start, &walls);

    let total = visited_cells.iter().filter(|&x| x).count();
    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (start, mut grid) = parse_input(input);
    let visited_cells = visited_route(start, &grid);

    let total = visited_cells
        .iter()
        .with_points()
        .filter(|(current, value)| (*value && *current != start))
        .filter(|(current, _)| {
            grid.set(*current, true);
            let count = is_circular_route(start, &grid);
            grid.set(*current, false);

            count
        })
        .count() as u64;

    Some(total)
}

/// Returns true if the route is circular
type PointProcessor<T> = fn(Point, Direction, &mut T) -> bool;

pub fn walk_route<T>(
    start: Point,
    walls: &Grid<bool>,
    state: &mut T,
    processor: PointProcessor<T>,
) {
    let mut next = start;

    'outer: for direction in DIRECTIONS.iter().cycle() {
        let mut iter = walls.iter().in_direction(*direction, next).with_points();

        loop {
            if let Some((current, wall)) = iter.next() {
                if wall {
                    break;
                }
                if processor(current, *direction, state) {
                    break 'outer;
                }
                next = current;
            } else {
                break 'outer;
            }
        }
    }
}

pub fn visited_route(start: Point, walls: &Grid<bool>) -> Grid<bool> {
    let mut visited_cells = Grid::<bool>::new_empty(walls.width, walls.height);

    walk_route(
        start,
        walls,
        &mut visited_cells,
        |point, _, visited_cells| {
            visited_cells.set(point, true);
            false
        },
    );

    visited_cells
}

pub fn is_circular_route(start: Point, walls: &Grid<bool>) -> bool {
    struct Package {
        visited: HashSet<(Point, Direction)>,
        is_circular: bool,
    }

    fn process_point(point: Point, direction: Direction, state: &mut Package) -> bool {
        if state.visited.contains(&(point, direction)) {
            state.is_circular = true;
            return true;
        }
        state.visited.insert((point, direction));
        false
    }

    let capacity = (walls.width * walls.height / 2) as usize;
    let visited = HashSet::<(Point, Direction)>::with_capacity(capacity);
    let is_circular = false;
    let mut package = Package {visited, is_circular };

    walk_route(start, walls, &mut package, process_point);

    package.is_circular
}

pub fn parse_input(input: &str) -> (Point, Grid<bool>) {
    let mut player = Point { x: 0, y: 0 };

    let walls: Vec<Vec<bool>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '#' {
                        return true;
                    } else if c == '^' {
                        player = Point {
                            x: x as isize,
                            y: y as isize,
                        };
                        return false;
                    }
                    false
                })
                .collect()
        })
        .collect();

    let walls_grid = Grid::new(walls);
    (player, walls_grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
