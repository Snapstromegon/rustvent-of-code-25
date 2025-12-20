use std::str::FromStr;

use crate::solution::{Solution, SolvedValue};

type Battery = u8;

fn max_battery_with_index(iter: &[Battery]) -> (usize, &Battery) {
    let mut max_i = 0;
    let mut max = &0;
    for (i, n) in iter.iter().enumerate() {
        if n > max {
            max_i = i;
            max = n;
        }
    }
    (max_i, max)
}

#[derive(Debug)]
struct Bank(Vec<Battery>);

impl Bank {
    fn max_power_2(&self) -> Battery {
        let (max_i, max) = max_battery_with_index(&self.0[0..(self.0.len() - 1)]);
        max * 10 + self.0.iter().skip(max_i + 1).max().unwrap()
    }
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bank(
            s.chars()
                .filter_map(|c| {
                    c.to_digit(10)
                        .map(|x| u8::try_from(x).expect("Unable to convert number"))
                })
                .collect(),
        ))
    }
}

fn parse_input(input: &str) -> Vec<Bank> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        Some(
            parse_input(input)
                .iter()
                .inspect(|i| print!("{i:?}"))
                .map(|bank| bank.max_power_2() as usize)
                .inspect(|s| println!(" = {s}"))
                .sum::<usize>()
                .into(),
        )
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

    const DAY: usize = 3;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(357.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(17109.into()));
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
