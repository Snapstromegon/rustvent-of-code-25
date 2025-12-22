use crate::solution::{Solution, SolvedValue};

fn parse_input(input: &str) -> Vec<((usize, usize), Vec<usize>)> {
    input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (size, counts) = line.split_once(": ").unwrap_or(("0x0", "0 0 0 0 0 0"));
            let (width, height) = size.split_once('x').unwrap();
            let size = (width.parse().unwrap(), height.parse().unwrap());
            let index_counts: Vec<usize> = counts
                .split_whitespace()
                .map(|count| count.parse().unwrap())
                .collect();
            (size, index_counts)
        })
        .collect()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let items = parse_input(input);
        let possibles = items
            .iter()
            .filter(|((width, height), counts)| {
                (width * height) >= counts.iter().sum::<usize>() * 9
            })
            .count();
        Some(possibles.into())
    }

    fn part2(&self, _input: &str) -> Option<SolvedValue> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 12;

    // #[test]
    // fn test_part1_example() {
    //     let input = read_input(DAY, true, 1).unwrap();
    //     assert_eq!(Day.part1(&input), None);
    // }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(519.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}
