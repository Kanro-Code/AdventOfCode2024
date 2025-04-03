use advent_of_code::{Grid, Point};
use itertools::Itertools;
use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (antennas, edge) = parse_input(input);
    let mut antinodes = Grid::<bool>::new_empty(edge.x, edge.y);

    antennas
        .values()
        .flat_map(|points| points.iter().combinations(2))
        .for_each(|pair| {
            let [p1, p2] = [pair[0], pair[1]];
            for (from, to) in [(p1, p2), (p2, p1)] {
                let offset = from.translate_offset(*to);
                antinodes.set_safe(offset, true);
            }
        });

    let total = antinodes.iter().filter(|&x| x).count() as u64;

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (antennas, edge) = parse_input(input);
    let mut antinodes = Grid::<bool>::new_empty(edge.x, edge.y);

    antennas
        .values()
        .flat_map(|points| points.iter().combinations(2))
        .for_each(|pair| {
            let [p1, p2] = [pair[0], pair[1]];
            for (from, to) in [(p1, p2), (p2, p1)] {
                let (dx, dy) = from.delta(*to);
                let mut to = *to;

                loop {
                    let offset = to.offset(dx, dy);
                    to = Point {
                        x: to.x + dx,
                        y: to.y + dy,
                    };
                    if !antinodes.set_safe(offset, true) {
                        break;
                    }
                }
            }
        });

    let total = antinodes.iter().filter(|&x| x).count() as u64;

    Some(total)
}

pub fn parse_input(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let mut map = HashMap::new();
    let x = (input.lines().next().unwrap().len()) as isize;
    let y = (input.lines().count()) as isize;

    let edge = Point { x, y };

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c != &'.')
            .for_each(|(x, c)| {
                map.entry(c).or_insert(Vec::new()).push(Point {
                    x: x as isize,
                    y: y as isize,
                });
            });
    });

    (map, edge)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
