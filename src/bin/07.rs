advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let operations = [u64::wrapping_add, u64::wrapping_mul];

    let total = get_total(input, &operations);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let operations = [u64::wrapping_add, u64::wrapping_mul, concatenate];

    let total = get_total(input, &operations);
    Some(total)
}

#[derive(Debug, Clone)]
pub struct Calculation {
    pub answer: u64,
    pub components: Vec<u64>,
}

type Operation = fn(u64, u64) -> u64;

impl Calculation {
    pub fn can_reach_answer(&self, operations: &[Operation]) -> bool {
        let number_encoding = operations.len().pow((self.components.len() as u32) - 1);

        for state in 0..number_encoding {
            let outcome = Self::apply_operations(
                self.components[0],
                &self.components[1..],
                state,
                operations,
            );

            if outcome == self.answer {
                return true;
            }
        }

        false
    }

    fn apply_operations(
        acc: u64,
        remaining_numbers: &[u64],
        state: usize,
        operations: &[Operation],
    ) -> u64 {
        if remaining_numbers.is_empty() {
            return acc;
        }

        let new = operations[state % operations.len()](acc, remaining_numbers[0]);
        let remaining = &remaining_numbers[1..];
        let operation_code = state / operations.len();
        Self::apply_operations(new, remaining, operation_code, operations)
    }
}

pub fn get_total(input: Vec<Calculation>, operations: &[Operation]) -> u64 {
    input
        .iter()
        .filter_map(|calc| match calc.can_reach_answer(operations) {
            true => Some(calc.answer),
            false => None,
        })
        .sum()
}

pub fn concatenate(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    let mut multiplier = 10;
    let mut temp = b;
    while temp >= 10 {
        multiplier *= 10;
        temp /= 10;
    }
    a * multiplier + b
}

pub fn parse_input(input: &str) -> Vec<Calculation> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let answer = left.parse().unwrap();
            let components = right
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            Calculation { answer, components }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
