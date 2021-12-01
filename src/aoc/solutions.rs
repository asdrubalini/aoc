use itertools::Itertools;

use super::Solution;

pub struct DayOne;

impl Solution for DayOne {
    type Output = usize;

    fn input() -> String {
        include_str!("./inputs/1.txt").to_string()
    }

    fn solve<S: AsRef<str>>(input: S) -> usize {
        let input = input.as_ref();

        // Split by line
        let items = input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap());

        items
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count()
    }

    fn expected_solution() -> Self::Output {
        1527
    }
}
