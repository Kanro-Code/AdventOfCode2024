advent_of_code::solution!(7);


const fn add(a: u64, b: u64) -> u64 { a + b }
const fn mul(a: u64, b: u64) -> u64 { a * b }
const OPERATIONS: [fn(u64, u64) -> u64; 2] = [add, mul];

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let total = input.iter().filter_map(|calc| {
        match calc.run() {
            true => Some(calc.answer),
            false => None
        }
    }).sum();

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[derive(Debug, Clone)]
pub struct Calculation {
    pub answer: u64,
    pub components: Vec<u64>,
}

impl Calculation {
    pub fn run(&self) -> bool {
        let count = (2_u64).pow((self.components.len() as u32) - 1) as usize;
        for state in 0..count {
            let outcome = recursive(self.components[0], &self.components[1..], state);

            if outcome == self.answer {
                return true;
            }

        }

        false
    }
}


pub fn recursive(total: u64, rest: &[u64], state: usize) -> u64 {
    if rest.is_empty() {
        return total;
    }

    let new = OPERATIONS[state % 2](total, rest[0]);
    recursive(new, &rest[1..], state / 2)
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
        assert_eq!(result, None);
    }
}
