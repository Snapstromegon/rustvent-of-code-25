use crate::solution::{Solution, SolvedValue};

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .filter_map(|line| line.split_once(','))
        .filter_map(|(a, b)| {
            a.parse()
                .and_then(|x| b.parse().map(|y| Position { x, y }))
                .ok()
        })
        .collect()
}

#[derive(Debug)]
struct Compressor {
    xs: Vec<usize>,
    ys: Vec<usize>,
}

impl Compressor {
    fn new(corners: &[Position]) -> Self {
        let mut compressed_x = Vec::new();
        let mut compressed_y = Vec::new();
        for &Position { x, y } in corners {
            compressed_x.push(x);
            compressed_y.push(y);
        }
        compressed_x.sort_unstable();
        compressed_x.dedup();
        compressed_y.sort_unstable();
        compressed_y.dedup();
        Compressor {
            xs: compressed_x,
            ys: compressed_y,
        }
    }

    fn compress_x(&self, x: usize) -> Option<usize> {
        self.xs.binary_search(&x).ok()
    }

    fn compress_y(&self, y: usize) -> Option<usize> {
        self.ys.binary_search(&y).ok()
    }

    fn compress(&self, pos: &Position) -> Option<Position> {
        Some(Position {
            x: self.compress_x(pos.x)?,
            y: self.compress_y(pos.y)?,
        })
    }

    fn decompress_x(&self, x: usize) -> Option<usize> {
        self.xs.get(x).copied()
    }

    fn decompress_y(&self, y: usize) -> Option<usize> {
        self.ys.get(y).copied()
    }

    fn decompress(&self, pos: &Position) -> Option<Position> {
        Some(Position {
            x: self.decompress_x(pos.x)?,
            y: self.decompress_y(pos.y)?,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let corners = parse_input(input);
        let mut max_area = 0;
        for (i, corner1) in corners.iter().enumerate() {
            for corner2 in corners.iter().skip(i + 1) {
                let area =
                    (corner1.x.abs_diff(corner2.x) + 1) * (corner1.y.abs_diff(corner2.y) + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        Some(max_area.abs_diff(0).into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let corners = parse_input(input);
        let compressor = Compressor::new(&corners);
        let compressed_corners: Vec<Position> = corners
            .iter()
            .map(|pos| compressor.compress(pos).unwrap())
            .collect();
        let mut grid = vec![vec![false; compressor.xs.len()]; compressor.ys.len()];
        let mut last_corner = compressed_corners[0];
        for &Position { x, y } in &compressed_corners {
            grid[y][x] = true;
            for row in grid
                .iter_mut()
                .take(last_corner.y.max(y) + 1)
                .skip(last_corner.y.min(y))
            {
                for cell in row
                    .iter_mut()
                    .take(last_corner.x.max(x) + 1)
                    .skip(last_corner.x.min(x))
                {
                    *cell = true;
                }
            }
            last_corner = Position { x, y };
        }
        for row in grid
            .iter_mut()
            .take(last_corner.y.max(compressed_corners[0].y) + 1)
            .skip(last_corner.y.min(compressed_corners[0].y))
        {
            for cell in row
                .iter_mut()
                .take(last_corner.x.max(compressed_corners[0].x) + 1)
                .skip(last_corner.x.min(compressed_corners[0].x))
            {
                *cell = true;
            }
        }

        let flow_start = Position {
            y: grid.len() / 4,
            x: grid[0].len() / 2,
        };
        grid[flow_start.y][flow_start.x] = true;
        let mut flow_positions = vec![flow_start];
        while let Some(Position { y: ry, x: rx }) = flow_positions.pop() {
            let directions = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in directions {
                let new_y = ry.cast_signed() + dy;
                let new_x = rx.cast_signed() + dx;
                if new_y >= 0
                    && new_y < grid.len().cast_signed()
                    && new_x >= 0
                    && new_x < grid[0].len().cast_signed()
                {
                    let (new_y, new_x) = (new_y.cast_unsigned(), new_x.cast_unsigned());
                    if !grid[new_y][new_x] {
                        grid[new_y][new_x] = true;
                        flow_positions.push(Position { y: new_y, x: new_x });
                    }
                }
            }
        }

        let mut max_area = 0;
        for (i, corner1) in compressed_corners.iter().enumerate() {
            for corner2 in compressed_corners.iter().skip(i + 1) {
                let (real_candidate1, real_candidate2) = (
                    compressor.decompress(corner1).unwrap(),
                    compressor.decompress(corner2).unwrap(),
                );
                let area = (real_candidate1.x.abs_diff(real_candidate2.x) + 1)
                    * (real_candidate1.y.abs_diff(real_candidate2.y) + 1);
                if grid
                    .iter()
                    .take(corner1.y.max(corner2.y) + 1)
                    .skip(corner1.y.min(corner2.y))
                    .all(|row| {
                        row.iter()
                            .take(corner1.x.max(corner2.x) + 1)
                            .skip(corner1.x.min(corner2.x))
                            .all(|&cell| cell)
                    })
                    && !compressed_corners.iter().any(|corner| {
                        corner.x > corner1.x.min(corner2.x)
                            && corner.x < corner1.x.max(corner2.x)
                            && corner.y > corner1.y.min(corner2.y)
                            && corner.y < corner1.y.max(corner2.y)
                    })
                    && area > max_area
                {
                    max_area = area;
                }
            }
        }
        Some(max_area.into())
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
        assert_eq!(Day.part2(&input), Some(24.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1_552_139_370.into()));
    }
}
