use std::{ops::RangeInclusive, str::FromStr};

use crate::solution::{Solution, SolvedValue};

fn do_ranges_overlap(range1: &RangeInclusive<usize>, range2: &RangeInclusive<usize>) -> bool {
    !(range1.end() < range2.start() || range2.end() < range1.start())
}

fn merge_overlapping_ranges(ranges: &[RangeInclusive<usize>]) -> Vec<RangeInclusive<usize>> {
    let mut merged_ranges: Vec<RangeInclusive<usize>> = ranges.to_vec();
    // It's possible that merging created new overlaps, so we need to repeat until no more merges occur
    let mut did_merge = true;
    while did_merge {
        did_merge = false;
        let mut new_merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();

        for range in &merged_ranges {
            let mut has_merged = false;
            for new_range in &mut new_merged_ranges {
                if do_ranges_overlap(new_range, range) {
                    let new_start = usize::min(*new_range.start(), *range.start());
                    let new_end = usize::max(*new_range.end(), *range.end());
                    *new_range = new_start..=new_end;
                    has_merged = true;
                    did_merge = true;
                    break;
                }
            }
            if !has_merged {
                new_merged_ranges.push(range.clone());
            }
        }

        merged_ranges = new_merged_ranges;
    }

    merged_ranges
}

struct Database {
    fresh: Vec<RangeInclusive<usize>>,
    items: Vec<usize>,
}

impl Database {
    fn count_fresh_items(&self) -> usize {
        self.items
            .iter()
            .filter(|&&item| self.fresh.iter().any(|range| range.contains(&item)))
            .count()
    }

    fn possible_items_in_fresh_ranges_count(&self) -> usize {
        merge_overlapping_ranges(&self.fresh)
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .sum::<usize>()
    }
}

impl FromStr for Database {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ranges, items) = s.split_once("\n\n").ok_or(())?;
        let fresh = ranges
            .lines()
            .filter_map(|line| {
                if let Some((start_str, end_str)) = line.split_once('-') {
                    let start = start_str.trim().parse().ok()?;
                    let end = end_str.trim().parse().ok()?;
                    Some(start..=end)
                } else {
                    None
                }
            })
            .collect();
        let items = items
            .lines()
            .filter_map(|line| line.trim().parse().ok())
            .collect();
        Ok(Database { fresh, items })
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        Some(input.parse::<Database>().ok()?.count_fresh_items().into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        Some(
            input
                .parse::<Database>()
                .ok()?
                .possible_items_in_fresh_ranges_count()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 5;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(3.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(690.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(14.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(344_323_629_240_733.into()));
    }
}
