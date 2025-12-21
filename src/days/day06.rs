use crate::solution::{Solution, SolvedValue};

#[derive(Debug)]
enum Equation {
    Add(Vec<usize>),
    Multiply(Vec<usize>),
}

impl Equation {
    fn execute(&self) -> usize {
        match self {
            Equation::Add(numbers) => numbers.iter().sum(),
            Equation::Multiply(numbers) => numbers.iter().product(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let (operators, number_lines) = lines.split_last().expect("Line missing");

    operators
        .iter()
        .enumerate()
        .map(|(index, op)| {
            let numbers = number_lines
                .iter()
                .filter_map(|number_line| number_line[index].trim().parse().ok())
                .collect();
            match *op {
                "+" => Equation::Add(numbers),
                "*" => Equation::Multiply(numbers),
                _ => panic!("Unknown operator"),
            }
        })
        .collect()
}

fn parse_transposed_input(input: &str) -> Vec<Equation> {
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let cols = lines[0].len();
    let mut transposed: Vec<String> = vec![String::with_capacity(lines.len()); cols];
    for col in &mut transposed {
        for line in &mut lines {
            col.push(line.remove(0));
        }
    }

    let mut groups = vec![];
    let mut current_group = vec![];

    for s in &mut transposed {
        *s = s.trim().to_string();
        if s.is_empty() {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = vec![];
            }
        } else {
            current_group.push(s.clone());
        }
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
        .iter_mut()
        .map(|group| {
            let group_type = group[0].pop().expect("Group cannot be empty");
            let numbers: Vec<usize> = group.iter().filter_map(|s| s.trim().parse().ok()).collect();
            if group_type == '+' {
                Equation::Add(numbers)
            } else {
                Equation::Multiply(numbers)
            }
        })
        .collect()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        Some(
            parse_input(input)
                .iter()
                .map(Equation::execute)
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        Some(
            parse_transposed_input(input)
                .iter()
                .map(Equation::execute)
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

    const DAY: usize = 6;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(4_277_556.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(5_977_759_036_837.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(3_263_827.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(9_630_000_828_442.into()));
    }
}
