advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let total: u64 = re.captures_iter(input).fold(0, |acc, captures| {
        let a = captures[1].parse::<u64>().unwrap();
        let b = captures[2].parse::<u64>().unwrap();
        acc + a * b
    });

    Some(total)
}
pub fn part_two(input: &str) -> Option<u64> {
    // Remove newlines
    let input = Regex::new(r"\n").unwrap().replace_all(input, "");
    // Remove all text between don't() and do()
    let input = Regex::new(r"don't\(\).*?do\(\)").unwrap().replace_all(&input, "");
    // Remove any text at the end of the string followed by don't()
    let input = Regex::new(r"don't\(\).*").unwrap().replace_all(&input, "");

    let total = part_one(&input).unwrap();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
