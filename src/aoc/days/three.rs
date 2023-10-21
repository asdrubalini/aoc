use core::panic;
use std::collections::hash_map;

use itertools::Itertools;

use crate::aoc::utils::{Coord, InfiniteMatrix};
use crate::aoc::Solution;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => panic!("lol"),
        }
    }
}

/// Infinite two dimensional houses grid
pub struct Three;

impl Solution for Three {
    type Output = u32;
    type Parsed = Vec<Direction>;

    fn input() -> &'static str {
        include_str!("../inputs/3.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.chars().map(Direction::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut pos = Coord(0, 0);
        let mut matrix = InfiniteMatrix::new(1u16);

        for direction in parsed {
            match direction {
                Direction::North => {
                    *pos.y_mut() += 1;
                }

                Direction::South => {
                    *pos.y_mut() -= 1;
                }

                Direction::East => {
                    *pos.x_mut() += 1;
                }

                Direction::West => {
                    *pos.x_mut() -= 1;
                }
            }

            *matrix.entry(pos).or_default() += 1;
        }

        matrix
            .into_iter()
            .map(|(_coord, count)| count)
            .filter(|count| *count >= 1)
            .count() as u32
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut santa_pos = Coord(0, 0);
        let mut robot_pos = Coord(0, 0);

        let mut matrix = InfiniteMatrix::new(2u16);

        for (i, direction) in parsed.iter().enumerate() {
            let curr_pos = if i % 2 == 0 {
                &mut santa_pos
            } else {
                &mut robot_pos
            };

            match direction {
                Direction::North => {
                    *curr_pos.y_mut() += 1;
                }

                Direction::South => {
                    *curr_pos.y_mut() -= 1;
                }

                Direction::East => {
                    *curr_pos.x_mut() += 1;
                }

                Direction::West => {
                    *curr_pos.x_mut() -= 1;
                }
            }

            *matrix.entry(*curr_pos).or_default() += 1;
        }

        matrix
            .into_iter()
            .map(|(_coord, count)| count)
            .filter(|count| *count >= 1)
            .count() as u32
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (2592, 0)
    }
}
