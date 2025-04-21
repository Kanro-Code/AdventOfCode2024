advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut total = 0;

    for e in input.iter() {
        transform_number(*e, 25, &mut total);
    }
    Some(total as u64)
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

pub fn transform_number(input: usize, steps: usize, total: &mut usize) {
    if steps == 0 {
        *total += 1;
        return;
    }

    let next_step = steps - 1;
    let count_digits = count_digits(input);

    if input == 0 {
        transform_number(1, next_step, total)
    } else if count_digits % 2 == 0 {
        let (a, b) = split_into_two(input, count_digits);

        transform_number(a, next_step, total);
        transform_number(b, next_step, total)
    } else {
        transform_number(input * 2024, next_step, total);
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
