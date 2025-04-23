use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let answer = blink_sequence(input, 25);
    Some(answer as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let answer = blink_sequence(input, 75);
    Some(answer as u64)
}

pub fn blink_sequence(input: Vec<usize>, iterations: usize) -> usize {
    let mut hash_map = HashMap::new();
    let mut end_nodes = 0;

    for e in input.iter() {
        let stone = Stone {
            number: *e,
            remaining: iterations,
        };
        end_nodes += blink(stone, &mut hash_map);
    }

    end_nodes
}

pub fn blink(stone: Stone, cache: &mut HashMap<Stone, usize>) -> usize {
    if stone.remaining == 0 {
        return 1;
    }

    if let Some(&cached) = cache.get(&stone) {
        return cached;
    }

    let count_digits = stone.count_digits();

    let answer = if stone.number == 0 {
        blink(stone.next(1), cache)
    } else if count_digits % 2 == 0 {
        let (half_a, half_b) = stone.split(count_digits / 2);
        blink(half_a, cache) + blink(half_b, cache)
    } else {
        blink(stone.next(stone.number * 2024), cache)
    };

    cache.insert(stone, answer);

    answer
}

pub fn split_into_two(input: usize, digits: usize) -> (usize, usize) {
    let half = digits / 2;
    let factor = 10_usize.pow(half as u32);
    let a = input / factor;
    let b = input % factor;
    (a, b)
}

pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
        })
        .collect()
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Stone {
    number: usize,
    remaining: usize,
}

impl Stone {
    pub fn next(&self, number: usize) -> Stone {
        Stone {
            number,
            remaining: self.remaining - 1,
        }
    }

    pub fn split(&self, split: usize) -> (Stone, Stone) {
        let factor = 10_usize.pow(split as u32);
        let a = self.number / factor;
        let b = self.number % factor;
        (
            Stone {
                number: a,
                remaining: self.remaining - 1,
            },
            Stone {
                number: b,
                remaining: self.remaining - 1,
            },
        )
    }

    pub fn count_digits(&self) -> usize {
        let mut count = 0;
        let mut x = self.number;
        while x > 0 {
            count += 1;
            x /= 10;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
