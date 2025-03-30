use advent_of_code::{Direction, Grid, Point};

advent_of_code::solution!(6);

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub fn part_one(input: &str) -> Option<u64> {
    let (start, grid) = parse_input(input);
    let (visited_cells, _) = visited_route(start, &grid);

    let total = visited_cells.iter().filter(|&x| x).count();
    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (start,mut grid) = parse_input(input);
    let (visited_cells, _) = visited_route(start, &grid);

    let total = visited_cells
        .iter()
        .with_points()
        .filter(|(current, value)| (*value && *current != start))
        .filter(|(current, _)| {
            grid.set(*current, true);
            let (_, outcome) = visited_route(start, &grid);
            grid.set(*current, false);

            outcome
        })
        .count() as u64;

    Some(total)
}

pub fn visited_route(start: Point, grid: &Grid<bool>) -> (Grid<bool>, bool) {
    let mut visited_cells = Grid::<bool>::new_empty(grid.width, grid.height);

    let mut current = start;
    let mut circular = false;
    let mut count = 0;

    'outer: for direction in DIRECTIONS.iter().cycle() {
        let mut iter = grid.iter().in_direction(*direction, current).with_points();

        loop {
            count += 1;
            if count > 10000 {
                circular = true;
                break 'outer;
            }
            if let Some((point, wall)) = iter.next() {
                if wall {
                    break;
                }
                visited_cells.set(point, true);
                current = point;
            } else {
                break 'outer;
            }
        }
    }

    (visited_cells, circular)
}

pub fn parse_input(input: &str) -> (Point, Grid<bool>) {
    let mut player = Point { x: 0, y: 0 };

    let ves: Vec<Vec<bool>> = input
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

    let grid = Grid::new(ves);
    (player, grid)
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
