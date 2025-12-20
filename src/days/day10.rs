use std::{collections::VecDeque, str::FromStr, usize};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::solution::{Solution, SolvedValue};

fn bool_vec_to_usize(vec: &[bool]) -> usize {
    vec.iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some(i) } else { None })
        .fold(0usize, |acc, idx| acc + (1 << idx))
}

type Button = Vec<usize>;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<usize>,
}

impl Machine {
    fn init_steps(&self) -> usize {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0usize));
        let goal = bool_vec_to_usize(&self.lights);
        let mut visited = std::collections::HashSet::new();

        while let Some((current_state, steps)) = queue.pop_front() {
            for button in &self.buttons {
                let mut new_state = current_state;
                for &idx in button {
                    new_state ^= 1 << idx;
                }
                if goal == current_state {
                    return steps;
                }
                if visited.insert(new_state) {
                    queue.push_back((new_state, steps + 1));
                }
            }
        }
        usize::MAX
    }

    fn joltages(&self) -> usize {
        let buttons_by_index: Vec<Vec<usize>> = self
            .buttons
            .iter()
            .enumerate()
            .flat_map(|(button_index, btn)| {
                btn.iter().map(move |&light_idx| (light_idx, button_index))
            })
            .fold(
                vec![Vec::new(); self.lights.len()],
                |mut acc, (light_idx, btn_idx)| {
                    acc[light_idx].push(btn_idx);
                    acc
                },
            );
        let mut light_order = buttons_by_index
            .iter()
            .map(Vec::len)
            .enumerate()
            .collect::<Vec<(usize, usize)>>();
        light_order.sort_by_key(|(_, button_count)| *button_count);
        let light_order = light_order
            .iter()
            .map(|(light_index, _)| *light_index)
            .collect::<Vec<usize>>();

        let mut candidates = VecDeque::new();
        candidates.push_back((vec![0; self.joltages.len()], self.buttons.clone()));

        usize::MAX
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lights, rest) = s.split_once(' ').ok_or(())?;
        let lights = lights[1..lights.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect();
        let (buttons_str, joltages_str) = rest.split_once(" {").ok_or(())?;
        let buttons = buttons_str
            .split_ascii_whitespace()
            .map(|btn_str| {
                btn_str
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .filter_map(|num_str| num_str.trim().parse().ok())
                    .collect()
            })
            .collect();
        let joltages = joltages_str
            .trim_end_matches('}')
            .split(',')
            .filter_map(|num_str| num_str.trim().parse().ok())
            .collect();
        Ok(Machine {
            lights,
            buttons,
            joltages,
        })
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let machines = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Machine>, _>>()
            .ok()?;
        Some(
            machines
                .par_iter()
                .map(Machine::init_steps)
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let _machines = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Machine>, _>>()
            .ok()?;
        // Some(machines.iter().map(Machine::joltages).sum::<usize>().into())
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 10;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(7.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(438.into()));
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
