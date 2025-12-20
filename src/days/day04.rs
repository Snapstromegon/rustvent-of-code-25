use std::str::FromStr;

use crate::solution::{Solution, SolvedValue};

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

struct Map(Vec<Vec<bool>>);

impl Map {
    fn number_of_neighbour_rolls(&self, pos: &Position) -> usize {
        // Bitmask as follows:
        // 012
        // 7X3
        // 654
        let mut bitmask: u8 = 0xff;
        if pos.row == 0 {
            bitmask &= 0b0001_1111;
        }
        if pos.col == self.0[0].len() - 1 {
            bitmask &= 0b1100_0111;
        }
        if pos.row == self.0[0].len() - 1 {
            bitmask &= 0b1111_0001;
        }
        if pos.col == 0 {
            bitmask &= 0b0111_1100;
        }
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ]
        .iter()
        .enumerate()
        .filter(|(index, _)| bitmask & (1 << (7 - index)) != 0)
        .filter(|(_, item)| {
            // println!("Position: {pos:?}, Item: {item:?}");
            // Safety:
            // Can only be valid here
            self.0[pos
                .row
                .checked_add_signed(item.0)
                .expect("Could not add to row")][pos
                .col
                .checked_add_signed(item.1)
                .expect("Could not add to col")]
        })
        .count()
    }

    fn reachable_roll_count(&self) -> usize {
        self.reachables().len()
    }

    fn reachables(&self) -> Vec<Position> {
        let mut reachables = vec![];
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                if self.0[row][col] && self.number_of_neighbour_rolls(&Position { row, col }) < 4 {
                    reachables.push(Position { row, col });
                }
            }
        }
        reachables
    }

    fn remove_reachables(&mut self) -> Option<usize> {
        let reachables = self.reachables();
        for reachable in &reachables {
            self.0[reachable.row][reachable.col] = false;
        }
        if reachables.is_empty() {
            None
        } else {
            Some(reachables.len())
        }
    }

    fn recursive_remove(&mut self) -> usize {
        let mut res = 0;
        while let Some(x) = self.remove_reachables() {
            res += x;
        }
        res
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect()))
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let map: Map = input.parse().expect("Couldn't parse Map");
        Some(map.reachable_roll_count().into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let mut map: Map = input.parse().expect("Couldn't parse Map");
        Some(map.recursive_remove().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 4;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(13.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1547.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(43.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(8948.into()));
    }
}
