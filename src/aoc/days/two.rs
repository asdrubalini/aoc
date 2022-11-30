use crate::aoc::Solution;

pub struct Two;

impl Solution for Two {
    type Output = usize;

    fn input() -> &'static str {
        ""
    }

    fn solve_first(_input: &str) -> Self::Output {
        0
    }

    fn solve_second(_input: &str) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}