use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, proposals) = parse_input(input);
    let map = parse_rules(rules);

    let total = proposals
        .iter()
        .filter_map(|proposal| {
            if check_proposal(&map, proposal) {
                let middle = proposal.len() / 2;
                Some(proposal[middle])
            } else {
                None
            }
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

pub fn check_proposal(rules: &HashMap<u64, Vec<u64>>, proposal: &[u64]) -> bool {
    // -1 because it doesn't need to check the last page
    for i in 0..proposal.len() - 1 {
        let current_page = proposal[i];

        let Some(forbidden_pages) = rules.get(&current_page) else {
            continue;
        };

        for j in i + 1..proposal.len() {
            if forbidden_pages.contains(&proposal[j]) {
                return false;
            }
        }
    }
    true
}

pub fn get_center(proposal: &[u64]) -> u64 {
    let middle = proposal.len() / 2;
    proposal[middle]
}

pub fn parse_rules(rules: Vec<Rule>) -> HashMap<u64, Vec<u64>> {
    rules.into_iter().fold(HashMap::new(), |mut map, rule| {
        map.entry(rule.second).or_default().push(rule.first);
        map
    })
}

pub fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<u64>>) {
    let (ordering, update) = input.split_once("\n\n").unwrap();

    let rules: Vec<Rule> = ordering
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            Rule {
                first: a.parse().unwrap(),
                second: b.parse().unwrap(),
            }
        })
        .collect();

    let proposals: Vec<Vec<u64>> = update
        .lines()
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    (rules, proposals)
}

#[derive(Debug)]
pub struct Rule {
    first: u64,
    second: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
