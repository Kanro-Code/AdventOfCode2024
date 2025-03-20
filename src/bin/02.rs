advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let total = input.iter().filter(|line| is_valid_input(line)).count() as u64;

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let total = input
        .iter()
        .filter(|line| is_valid_input_tolerance(line))
        .count() as u64;

    Some(total)
}

pub fn is_valid_input_tolerance(input: &[u8]) -> bool {
    if is_valid_input(input) {
        return true;
    }

    // Removes a single element from the list, checks it against is_valid_input.
    // If any result is true, exits early and returns true.
    for i in 0..input.len() {
        let new_list: Vec<u8> = input
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, e)| *e)
            .collect();

        if is_valid_input(&new_list) {
            return true;
        }
    }

    false
}

pub fn is_valid_input(input: &[u8]) -> bool {
    is_sorted_asc_or_desc(input) && is_valid_distance(input)
}

pub fn is_sorted_asc_or_desc(input: &[u8]) -> bool {
    input.is_sorted() || input.is_sorted_by(|a, b| a >= b)
}

pub fn is_valid_distance(input: &[u8]) -> bool {
    input
        .windows(2)
        .all(|pair| matches!(pair, &[a, b] if is_valid_pair(a, b)))
}

pub fn is_valid_pair(a: u8, b: u8) -> bool {
    let diff = a.abs_diff(b);
    diff > 0 && diff <= 3
}

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
