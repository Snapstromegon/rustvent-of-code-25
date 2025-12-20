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
    fn max_power_2(&self) -> usize {
        let (max_i, max) = max_battery_with_index(&self.0[0..(self.0.len() - 1)]);
        *max as usize * 10 + *self.0.iter().skip(max_i + 1).max().unwrap() as usize
    }

    fn max_power_12(&self) -> usize {
        let mut start = 0;
        let mut res = 0;
        for i in 0..12 {
            let end = self.0.len() - 12 + i;
            let (max_i, max) = max_battery_with_index(&self.0[start..=end]);
            start += max_i + 1;
            res = res * 10 + *max as usize;
        }
        res
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
                .map(Bank::max_power_2)
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        Some(
            parse_input(input)
                .iter()
                // .inspect(|x| print!("{x:?}"))
                .map(Bank::max_power_12)
                // .inspect(|x| println!("\t{x:?}"))
                .sum::<usize>()
                .into(),
        )
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
        assert_eq!(Day.part2(&input), Some(3_121_910_778_619.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(169_347_417_057_382.into()));
    }
}
