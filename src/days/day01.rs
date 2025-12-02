use std::str::FromStr;

use anyhow::anyhow;

use crate::solution::{Solution, SolvedValue};

struct Dial {
    position: isize,
}

impl Dial {
    fn new() -> Self {
        Self { position: 50 }
    }

    fn rotate(&mut self, rotation: &Rotation) -> usize {
        let full_rotations = (rotation.amount() / 100).abs() as usize;
        let part_rotation = rotation.amount() - 100 * full_rotations as isize;
        let starts_at_0 = self.position == 0;
        match rotation {
            Rotation::L(_) => {
                self.position = self.position - part_rotation;
            }
            Rotation::R(_) => {
                self.position = self.position + part_rotation;
            }
        }
        let mut crossed = full_rotations;
        if !starts_at_0 {
            if self.position == 0 {
                crossed += 1;
            }
            if self.position < 0 {
                crossed += 1;
            } else if self.position >= 100 {
                crossed += 1;
            }
        }
        if self.position < 0 {
            self.position += 100;
        } else if self.position >= 100 {
            self.position -= 100;
        }

        return crossed;
    }
}

enum Rotation {
    L(isize),
    R(isize),
}

impl Rotation {
    fn amount(&self) -> isize {
        match self {
            Rotation::L(steps) => *steps,
            Rotation::R(steps) => *steps,
        }
    }
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s.split_at(1);
        let steps: isize = steps.parse()?;
        match dir {
            "L" => Ok(Rotation::L(steps)),
            "R" => Ok(Rotation::R(steps)),
            _ => Err(anyhow!("Invalid rotation direction {dir}")),
        }
    }
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    input
        .trim()
        .lines()
        .map(|s| s.parse().expect("Failed to parse rotation"))
        .collect()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let rotations = parse_rotations(input);
        let mut dial = Dial::new();
        let mut times_0_reached = 0;
        for rotation in rotations {
            dial.rotate(&rotation);
            if dial.position == 0 {
                times_0_reached += 1;
            }
        }
        Some(times_0_reached.into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let rotations = parse_rotations(input);
        let mut dial = Dial::new();
        let mut times_0_passed = 0;
        for rotation in rotations {
            times_0_passed += dial.rotate(&rotation);
        }
        Some(times_0_passed.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 1;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(3.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1011.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(6.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(5937.into()));
    }
}
