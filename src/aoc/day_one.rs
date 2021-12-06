use itertools::Itertools;

use super::Solution;

pub struct DayOne;

impl Solution for DayOne {
    type Output = usize;

    fn input() -> &'static str {
        include_str!("./inputs/1.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        // Split by line
        let items = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u16>().unwrap());

        items
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count()
    }

    fn solve_second(input: &str) -> Self::Output {
        // Split by line
        let items = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u16>().unwrap());

        items
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1527, 1575)
    }
}
