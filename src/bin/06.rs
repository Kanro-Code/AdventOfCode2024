use advent_of_code::{Direction, Grid, Point};

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
    let (start,mut grid) = parse_input(input);
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

type PointProcessor<T> = fn(Point, &mut T);

pub fn walk_route<T>(start: Point, walls: &Grid<bool>, state: &mut T, processor: PointProcessor<T>) {
    let mut next = start;

    let mut TEMP_COUNTER = 0;

    processor(next, state);

    'outer: for direction in DIRECTIONS.iter().cycle() {
        let mut iter = walls.iter().in_direction(*direction, next).with_points();

        loop {
            if let Some((current, wall)) = iter.next() {
                TEMP_COUNTER += 1;
                if TEMP_COUNTER > 100000 {
                    break 'outer;
                }
                if wall {
                    break;
                }
                processor(current, state);
                next = current;
            } else {
                break 'outer;
            }
        }
    }
}

pub fn visited_route(start: Point, walls: &Grid<bool>) -> Grid<bool> {
    let mut visited_cells = Grid::<bool>::new_empty(walls.width, walls.height);

    walk_route(start, walls, &mut visited_cells, |point, visited_cells| {
        visited_cells.set(point, true);
    });

    visited_cells
}

pub fn is_circular_route(start: Point, walls: &Grid<bool>) -> bool {
    let mut counter = 0;

    walk_route(start, walls, &mut counter, |_, counter| {
        *counter += 1;
    });

    counter > 20000
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
