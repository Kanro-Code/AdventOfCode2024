use advent_of_code::{Coordinate, Grid};

advent_of_code::solution!(6);

pub fn part_one(_input: &str) -> Option<u64> {
    let (player, grid) = parse_input(_input);
    println!("{player:?} {grid:?}");
    None
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

pub fn parse_input(input: &str) -> (Coordinate, Grid<bool>) {
    let mut player: Coordinate = Coordinate { x: usize::MAX, y: usize::MAX };

    let ves: Vec<Vec<bool>> = input
        .lines().enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| {
                if c == '#' {
                    return true;
                } else if c == '^' {
                    player = Coordinate { x, y };
                    return true;
                }
                false
            }).collect()
        }).collect();

    let grid = Grid::new(ves);
    (player, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
