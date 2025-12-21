use crate::solution::{Solution, SolvedValue};

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .filter_map(|line| line.split_once(','))
        .filter_map(|(a, b)| a.parse().and_then(|x| b.parse().map(|y| (x, y))).ok())
        .collect()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let corners = parse_input(input);
        let mut max_area = 0;
        for (i, corner1) in corners.iter().enumerate() {
            for corner2 in corners.iter().skip(i+1) {
                let area = ((corner2.0 - corner1.0).abs() + 1) * ((corner2.1 - corner1.1).abs() + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        Some(max_area.abs_diff(0).into())
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

    const DAY: usize = 9;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(50.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(4_746_238_001.into()));
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
