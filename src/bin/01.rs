use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);
    let total: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);

    let frequencies: HashMap<u64, u64> = right.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    let total = left
        .iter()
        .map(|a| a * frequencies.get(a).unwrap_or(&0))
        .sum();

    Some(total)
}

pub fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        list1.push(a.parse().unwrap());
        list2.push(b.parse().unwrap());
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
