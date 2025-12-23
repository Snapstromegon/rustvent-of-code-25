use std::str::FromStr;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use z3::{Optimize, ast::Int};

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
        let optimizer = Optimize::new();

        let buttons: Vec<Int> = (0..self.buttons.len())
            .map(|btn| {
                let button = Int::fresh_const(&format!("button_{btn}"));
                optimizer.assert(&button.ge(0));
                button
            })
            .collect();

        for (i, joltage) in self.joltages.iter().enumerate() {
            let needed_buttons = self
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(btn_idx, btn)| {
                    if btn.contains(&i) {
                        Some(&buttons[btn_idx])
                    } else {
                        None
                    }
                });
            optimizer.assert(
                &needed_buttons
                    .sum::<Int>()
                    .eq(u32::try_from(*joltage).unwrap()),
            );
        }

        let button_sum = buttons.iter().sum::<Int>();

        optimizer.minimize(&button_sum);

        // Get the minimum value for button_sum
        match optimizer.check(&[]) {
            z3::SatResult::Sat => {
                let model = optimizer.get_model().unwrap();
                let value = model.eval(&button_sum, true).unwrap();
                usize::try_from(value.as_u64().unwrap()).unwrap()
            }
            _ => usize::MAX,
        }
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
        let machines = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Machine>, _>>()
            .ok()?;
        Some(machines.iter().map(Machine::joltages).sum::<usize>().into())
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
        assert_eq!(Day.part2(&input), Some(33.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(16_463.into()));
    }
}
