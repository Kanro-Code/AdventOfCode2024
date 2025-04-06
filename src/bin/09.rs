advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = parse_input(input);
    let mut end = input.len() - 1;
    let capacity = input.iter().filter_map(|b| {
        b.id?;
        Some(b.size)
    }).sum::<usize>();

    let mut answer = Vec::with_capacity(capacity);


    for i in 0..input.len() {
        if answer.len() == capacity {
            break;
        }
        let entry = input[i];

        if entry.id.is_some() {
            for _ in 0..entry.size {
                answer.push(entry.id.unwrap());
            }
        } else {
            let mut size = entry.size;

            while size > 0 {
                if input[end].id.is_none() {
                    end -= 1;
                    continue;
                }

                if input[end].size > 0 {
                    input[end].size -= 1;

                    answer.push(input[end].id.unwrap());
                    size -= 1;
                } else {
                    end -= 1;
                }
            }
        }
    }

    let total = checksum(&answer);

    Some(total as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

pub fn checksum(input: &[usize]) -> usize {
    input.iter().enumerate().fold(0, |acc, (i, &x)| {
        acc + (i * x)
    })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Block {
    pub id: Option<usize>,
    pub size: usize,
}

pub fn parse_input(input: &str) -> Vec<Block> {
    input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if size == 0 {
                return None;
            }

            if i % 2 == 0 {
                let id = i / 2;
                Some(Block { id: Some(id), size })
            } else {
                let id = None;
                Some(Block { id, size })
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
