use crate::aoc::Solution;

pub struct TwentyOne;

impl Solution for TwentyOne {
    type Output = usize;
    type Parsed = Vec<u32>;

    fn input() -> &'static str {
        ""
    }

    fn parse_input(_input: &'static str) -> Self::Parsed {
        vec![]
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