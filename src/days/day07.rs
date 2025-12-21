use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use crate::solution::{Solution, SolvedValue};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    col: usize,
    row: usize,
}

#[derive(Debug)]
struct Map {
    start: Position,
    splitters: Vec<Vec<bool>>,
}

impl Map {
    fn number_of_splits(&self) -> usize {
        let mut queue = VecDeque::from(vec![self.start]);
        let mut visited = HashSet::new();
        let mut splits = 0;

        while let Some(item) = queue.pop_front() {
            if !visited.contains(&item) {
                visited.insert(item);
                if item.row < self.splitters.len() {
                    if self.is_splitter_below(item) {
                        splits += 1;
                        queue.push_back(Position {
                            col: item.col - 1,
                            row: item.row + 1,
                        });
                        queue.push_back(Position {
                            col: item.col + 1,
                            row: item.row + 1,
                        });
                    } else {
                        queue.push_back(Position {
                            row: item.row + 1,
                            col: item.col,
                        });
                    }
                }
            }
        }
        splits
    }
    fn is_splitter_at(&self, pos: Position) -> bool {
        self.splitters
            .get(pos.row)
            .unwrap_or(&vec![])
            .get(pos.col)
            .unwrap_or(&false)
            .to_owned()
    }
    fn is_splitter_below(&self, pos: Position) -> bool {
        self.is_splitter_at(Position {
            col: pos.col,
            row: pos.row + 1,
        })
    }

    fn number_of_timeline_splits(&self) -> usize {
        let mut possible_paths = vec![vec![0usize; self.splitters[0].len()]; self.splitters.len()];
        possible_paths[self.start.row][self.start.col] = 1;
        for row in 1..self.splitters.len() {
            for col in 0..self.splitters[row].len() {
                if self.is_splitter_at(Position { col, row }) {
                    possible_paths[row][col - 1] += possible_paths[row - 1][col];
                    possible_paths[row][col + 1] += possible_paths[row - 1][col];
                } else {
                    possible_paths[row][col] += possible_paths[row - 1][col];
                }
            }
        }
        possible_paths[self.splitters.len() - 1].iter().sum()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row, line) in self.splitters.iter().enumerate() {
            for (col, is_splitter) in line.iter().enumerate() {
                let curr_pos = Position { col, row };
                if self.start == curr_pos {
                    write!(f, "S")?;
                } else if *is_splitter {
                    write!(f, "^")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Position { col: 0, row: 0 };
        let mut splitters = vec![];
        for (row, line) in s.lines().enumerate() {
            let mut line_splitters = vec![];
            for (col, c) in line.trim().chars().enumerate() {
                line_splitters.push(match c {
                    'S' => {
                        start = Position { col, row };
                        false
                    }
                    '^' => true,
                    _ => false,
                });
            }
            splitters.push(line_splitters);
        }

        Ok(Map { start, splitters })
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        Some(
            input
                .parse::<Map>()
                .expect("Unable to parse map")
                .number_of_splits()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        Some(
            input
                .parse::<Map>()
                .expect("Unable to parse map")
                .number_of_timeline_splits()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 7;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(21.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1687.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(40.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(390_684_413_472_684.into()));
    }
}
