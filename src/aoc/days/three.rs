use core::panic;

use itertools::Itertools;

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
        input.chars().into_iter().map(Direction::from).collect_vec()
    }

    fn solve_first(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
