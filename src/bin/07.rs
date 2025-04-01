advent_of_code::solution!(7);


fn add(a: u64, b: u64) -> u64 { a + b }
fn mul(a: u64, b: u64) -> u64 { a * b }
fn concatenate(a: u64, b: u64) -> u64 {
    let a = a.to_string();
    let b = b.to_string();
    a.chars().chain(b.chars()).collect::<String>().parse().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let operations = [add, mul];

    let input = parse_input(input);

    let total = input.iter().filter_map(|calc| {
        match calc.run(&operations) {
            true => Some(calc.answer),
            false => None
        }
    }).sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations = [add, mul, concatenate];

    let input = parse_input(input);

    let total = input.iter().filter_map(|calc| {
        match calc.run(&operations) {
            true => Some(calc.answer),
            false => None
        }
    }).sum();

    Some(total)
}

#[derive(Debug, Clone)]
pub struct Calculation {
    pub answer: u64,
    pub components: Vec<u64>,
}

type Operation = fn(u64, u64) -> u64;

impl Calculation {
    pub fn run(&self, operations: &[Operation]) -> bool {
        let count = operations.len().pow((self.components.len() as u32) - 1);
        for state in 0..count {
            let outcome = recursive(self.components[0], &self.components[1..], state, operations);

            if outcome == self.answer {
                return true;
            }

        }

        false
    }
}


pub fn recursive(total: u64, rest: &[u64], state: usize, operations: &[Operation]) -> u64 {
    if rest.is_empty() {
        return total;
    }

    let new = operations[state % operations.len()](total, rest[0]);
    recursive(new, &rest[1..], state / operations.len(), operations)
}

pub fn parse_input(input: &str) -> Vec<Calculation> {
    input.lines().map(|line| {
        let (left, right) = line.split_once(": ").unwrap();
        let answer = left.parse().unwrap();
        let components = right.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Calculation { answer, components }
    }).collect()
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
