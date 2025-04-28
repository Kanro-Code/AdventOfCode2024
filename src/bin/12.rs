advent_of_code::solution!(12);
use advent_of_code::{Direction, Grid};

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let results = input.iter().with_points().filter_map(|(point, plot)| {
        if plot.checked {
            return None;
        }

        let points = input.get_cluster(point);

        let mut fence = 0;
        let surface = points.len() as u64;

        for point in points {
            for direction in DIRECTIONS {
                let next = point.translate_direction(direction);
                let val = input.get_safe(next);
                if val.is_none() || val.unwrap().plant != plot.plant {
                    fence += 1;
                }
            }
        }


        Some(fence * surface)

    });
    Some(results.sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

const DIRECTIONS: [Direction; 4] =  [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub fn parse_input(input: &str) -> Grid<Plot> {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    Plot {
                        checked: false,
                        plant: c,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Grid::new(data)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Plot {
	checked: bool,
    plant: char,

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
