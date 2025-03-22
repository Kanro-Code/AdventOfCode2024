use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (rulebook, proposals) = parse_input(input);

    let total = proposals
        .iter()
        .map(|proposal| {
            if rulebook.is_valid_proposal(proposal) {
                proposal.center()
            } else {
                0
            }
        })
        .sum();

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[derive(Debug)]
pub struct Rule {
    first: u64,
    second: u64,
}

pub struct RuleBook {
    map: HashMap<u64, Vec<u64>>,
}

impl RuleBook {
    pub fn new(rules: Vec<Rule>) -> Self {
        let map = Self::parse_rules(rules);
        Self { map }
    }

    pub fn is_valid_proposal(&self, proposal: &Proposal) -> bool {
        // -1 because it doesn't need to check the last page
        for i in 0..proposal.len() - 1 {
            let current_page = proposal.inner[i];

            let Some(forbidden_pages) = self.map.get(&current_page) else {
                continue;
            };

            for j in i + 1..proposal.len() {
                if forbidden_pages.contains(&proposal.inner[j]) {
                    return false;
                }
            }
        }
        true
    }

    pub fn parse_rules(rules: Vec<Rule>) -> HashMap<u64, Vec<u64>> {
        rules.into_iter().fold(HashMap::new(), |mut map, rule| {
            map.entry(rule.second).or_default().push(rule.first);
            map
        })
    }
}

pub struct Proposal {
    inner: Vec<u64>,
}

impl Proposal {
    pub fn new(inner: Vec<u64>) -> Self {
        Self { inner }
    }

    pub fn center(&self) -> u64 {
        self.inner[self.inner.len() / 2]
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

pub fn parse_input(input: &str) -> (RuleBook, Vec<Proposal>) {
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

    let rulebook = RuleBook::new(rules);

    let proposals: Vec<Proposal> = update
        .lines()
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .map(Proposal::new)
        .collect();

    (rulebook, proposals)
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
