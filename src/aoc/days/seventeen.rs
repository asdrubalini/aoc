use itertools::Itertools;

use crate::aoc::Solution;

pub struct Seventeen;

impl Solution for Seventeen {
    type Output = u32;
    type Parsed = Vec<u32>;

    fn input() -> &'static str {
        include_str!("../inputs/17.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(|n| n.parse().unwrap()).collect_vec()
    }

    fn solve_first(containers: &Self::Parsed) -> Self::Output {
        0
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
