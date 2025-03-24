use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (rulebook, proposals) = parse_input(input);

    let total = proposals.iter().fold(0, |acc, proposal| {
        if rulebook.is_valid(proposal) {
            return acc + proposal.center();
        }
        acc
    });

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rulebook, proposals) = parse_input(input);

    let total: u64 = proposals.iter().fold(0, |acc, proposal| {
        if !rulebook.is_valid(proposal) {
            let fixed = rulebook.fix_proposal(proposal);
            acc + fixed.center()
        } else {
            acc
        }
    });
    Some(total)
}

#[derive(Debug, Clone)]
pub struct Rule {
    first: u64,
    second: u64,
}

#[derive(Debug, Clone)]
pub struct RuleBook {
    map: HashMap<u64, Vec<u64>>,
}

impl RuleBook {
    pub fn is_valid(&self, proposal: &Proposal) -> bool {
        proposal.inner.windows(2).all(|pair| {
            let [current, next] = pair else {
                return false;
            };
            self.compare_pages(*current, *next) != std::cmp::Ordering::Less
        })
    }

    pub fn fix_proposal(&self, proposal: &Proposal) -> Proposal {
        let mut fixed = proposal.clone();
        fixed
            .inner
            .sort_by(|&page1, &page2| self.compare_pages(page1, page2));

        fixed
    }

    fn compare_pages(&self, a: u64, b: u64) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match (self.map.get(&a), self.map.get(&b)) {
            (Some(a_forbids), Some(b_forbids)) => {
                if a_forbids.contains(&b) {
                    Ordering::Less
                } else if b_forbids.contains(&a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (Some(a_forbids), None) if a_forbids.contains(&b) => Ordering::Less,
            (None, Some(b_forbids)) if b_forbids.contains(&a) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl Into<RuleBook> for Vec<Rule> {
    fn into(self) -> RuleBook {
        let map = self
            .into_iter()
            .fold(HashMap::new(), |mut map: HashMap<u64, Vec<u64>>, rule| {
                map.entry(rule.first).or_default().push(rule.second);
                map
            });
        RuleBook { map }
    }
}

#[derive(Debug, Clone)]
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

    pub fn iter(&self) -> impl Iterator<Item = &u64> {
        self.inner.iter()
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

    let rulebook = rules.into();

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
        assert_eq!(result, Some(143 + 123));
    }
}
