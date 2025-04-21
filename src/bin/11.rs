advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut end_nodes = 0;

    for e in input.iter() {
        blink(*e, 25, &mut end_nodes);
    }
    Some(end_nodes as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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

pub fn blink(number: usize, remaining_blinks: usize, end_nodes: &mut usize) {
    if remaining_blinks == 0 {
        *end_nodes += 1;
        return;
    }

    let remaining_blinks = remaining_blinks - 1;
    let count_digits = count_digits(number);

    if number == 0 {
        blink(1, remaining_blinks, end_nodes)
    } else if count_digits % 2 == 0 {
        let (a, b) = split_into_two(number, count_digits);

        blink(a, remaining_blinks, end_nodes);
        blink(b, remaining_blinks, end_nodes)
    } else {
        blink(number * 2024, remaining_blinks, end_nodes);
    }
}

pub fn split_into_two(input: usize, digits: usize) -> (usize, usize) {
    let half = digits / 2;
    let factor = 10_usize.pow(half as u32);
    let a = input / factor;
    let b = input % factor;
    (a, b)
}

pub fn count_digits(input: usize) -> usize {
    let mut count = 0;
    let mut x = input;
    while x > 0 {
        count += 1;
        x /= 10;
    }
    count
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
        assert_eq!(result, None);
    }
}
