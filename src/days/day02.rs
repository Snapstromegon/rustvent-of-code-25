use std::ops::RangeInclusive;

use crate::solution::{Solution, SolvedValue};

fn parse_input_to_ranges(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .split(',')
        .filter_map(|line| {
            if let Some((start_str, end_str)) = line.split_once('-') {
                let start = start_str.trim().parse().ok()?;
                let end = end_str.trim().parse().ok()?;
                Some(start..=end)
            } else {
                None
            }
        })
        .collect()
}

fn get_base10_half_number(number: usize, rounding: u32) -> usize {
    if number < 10 {
        1
    } else {
        number / 10_usize.pow((number.ilog10() + rounding).div_ceil(2))
    }
}

fn find_doubles_in_range(range: &RangeInclusive<usize>) -> Vec<usize> {
    let half_start = get_base10_half_number(*range.start(), 1);
    let half_end = get_base10_half_number(*range.end(), 0);

    (half_start..=half_end)
        .map(|half| half * 10_usize.pow(half.ilog10() + 1) + half)
        .filter(|n| range.contains(n))
        .collect()
}

fn has_number_repetitions(number: usize) -> bool {
    // A repetition is a pattern like 11241124 or 123123123
    let num_as_str = number.to_string();
    let chars: Vec<char> = num_as_str.chars().collect();
    for i in (1..=(chars.len() / 2)).filter(|i| chars.len().is_multiple_of(*i)) {
        let pattern = &chars[0..i];
        if chars.chunks(i).all(|chunk| chunk == pattern) {
            return true;
        }
    }
    false
}

fn get_range_repitions(range: &RangeInclusive<usize>) -> Vec<usize> {
    range
        .clone()
        .filter(|n| has_number_repetitions(*n))
        .collect()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        Some(
            parse_input_to_ranges(input)
                .iter()
                .flat_map(find_doubles_in_range)
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        Some(
            parse_input_to_ranges(input)
                .iter()
                .flat_map(get_range_repitions)
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

    const DAY: usize = 2;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), None);
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), None);
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(4_174_379_265.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(11_323_661_261.into()));
    }
}
