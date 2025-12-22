use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

use crate::solution::{Solution, SolvedValue};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Position {
    fn distance(&self, other: &Position) -> usize {
        let (dx, dy, dz) = (
            self.x.abs_diff(other.x),
            self.y.abs_diff(other.y),
            self.z.abs_diff(other.z),
        );
        (dx * dx + dy * dy + dz * dz).isqrt()
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .splitn(3, ',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<usize>>()
            .as_slice()
        {
            &[x, y, z] => Ok(Self { x, y, z }),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Distance<'a> {
    a: &'a Position,
    b: &'a Position,
    dist: usize,
}

impl<'a> Distance<'a> {
    fn new(a: &'a Position, b: &'a Position) -> Self {
        Distance {
            a,
            b,
            dist: a.distance(b),
        }
    }
}

impl PartialOrd for Distance<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

fn parse_input(input: &str) -> Vec<Position> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

fn build_distance_heap(positions: &[Position]) -> BinaryHeap<Reverse<Distance<'_>>> {
    let mut res = BinaryHeap::new();

    for (i, a) in positions.iter().enumerate() {
        for b in positions.iter().skip(i + 1) {
            res.push(Reverse(Distance::new(a, b)));
        }
    }

    res
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let positions = parse_input(input);
        let mut distances = build_distance_heap(&positions);
        let mut cliques: HashSet<Vec<Position>> = HashSet::new();

        let max_iter = match positions.len() {
            20 => 10,
            _ => 1000,
        };

        for _ in 0..max_iter {
            let distance = distances.pop().expect("Distances Empty");
            let mut new_clique: HashSet<Position> = HashSet::new();
            new_clique.insert(*distance.0.a);
            new_clique.insert(*distance.0.b);
            cliques.retain(|c| {
                if c.contains(distance.0.a) || c.contains(distance.0.b) {
                    new_clique.extend(c);
                    false
                } else {
                    true
                }
            });
            cliques.insert(new_clique.into_iter().collect());
        }

        let mut cliques = cliques.into_iter().collect::<Vec<Vec<Position>>>();
        cliques.sort_by_key(Vec::len);
        Some(
            cliques
                .iter()
                .rev()
                .take(3)
                .map(Vec::len)
                .product::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let positions = parse_input(input);
        let mut distances = build_distance_heap(&positions);
        let mut cliques: HashSet<Vec<Position>> = HashSet::new();

        let mut distance=distances.peek().unwrap().clone();

        while cliques.len() != 1 || cliques.iter().next().map(Vec::len) != Some(positions.len()) {
            distance = distances.pop().expect("Distances Empty");
            let mut new_clique: HashSet<Position> = HashSet::new();
            new_clique.insert(*distance.0.a);
            new_clique.insert(*distance.0.b);
            cliques.retain(|c| {
                if c.contains(distance.0.a) || c.contains(distance.0.b) {
                    new_clique.extend(c);
                    false
                } else {
                    true
                }
            });
            cliques.insert(new_clique.into_iter().collect());
        }
        Some(
            (distance.0.a.x * distance.0.b.x).into()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 8;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(40.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(171_503.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(25_272.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(9_069_509_600.into()));
    }
}
