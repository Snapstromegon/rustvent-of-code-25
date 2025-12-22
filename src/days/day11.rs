use std::{collections::HashMap, sync::RwLock};

use crate::solution::{Solution, SolvedValue};

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(": ").unwrap();
            let outputs = output.split_whitespace().collect();
            (input, outputs)
        })
        .collect()
}

fn get_paths_count_to_out<'a>(
    start: &'a str,
    goal: &'a str,
    machines: &'a HashMap<&'a str, Vec<&'a str>>,
    memo: &RwLock<HashMap<&'a str, usize>>,
) -> usize {
    if start == goal {
        return 1;
    }
    if let Some(count) = memo.read().unwrap().get(start) {
        return *count;
    }
    let mut total_paths = 0;
    if let Some(neighbors) = machines.get(start) {
        for &neighbor in neighbors {
            total_paths += get_paths_count_to_out(neighbor, goal, machines, memo);
        }
    }
    memo.write().unwrap().insert(start, total_paths);
    total_paths
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let machines = parse_input(input);
        let res = get_paths_count_to_out("you", "out", &machines, &RwLock::new(HashMap::new()));
        Some(res.into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let machines = parse_input(input);
        // We know that the order has to be either svr -> fft -> dac -> out or svr -> dac -> fft -> out,
        // because if fft -> dac -> fft or dac -> fft -> dac happens, there would be cycles and infinite paths.
        // By trying it, we find that only the first one yields paths.
        let svr_fft = get_paths_count_to_out("svr", "fft", &machines, &RwLock::new(HashMap::new()));
        let fft_dac = get_paths_count_to_out("fft", "dac", &machines, &RwLock::new(HashMap::new()));
        let dac_out = get_paths_count_to_out("dac", "out", &machines, &RwLock::new(HashMap::new()));
        Some((svr_fft * fft_dac * dac_out).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 11;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(5.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(555.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(2.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(502_447_498_690_860.into()));
    }
}
