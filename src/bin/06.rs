use advent_of_code::{Direction, Grid, Point};

advent_of_code::solution!(6);

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub fn part_one(input: &str) -> Option<u64> {
    let (player, grid) = parse_input(input);

    let mut visited_cells = Grid::<bool>::new_empty(grid.width, grid.height);
    visited_cells.set(player, true);

    let mut iter = grid
        .iter()
        .in_direction(DIRECTIONS[0], player)
        .with_points();
    let mut turns = 0;
    let mut previous: Point = player;

    while let Some((point, wall)) = iter.next() {
        if wall {
            turns += 1;
            iter = grid
                .iter()
                .in_direction(DIRECTIONS[turns % 4], previous)
                .with_points();
        } else {
            visited_cells.set(point, true);
            previous = point;
        }
    }

    let total = visited_cells.iter().filter(|&x| x).count();
    Some(total as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
