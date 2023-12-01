use std::iter::Sum;

use itertools::Itertools;
use serde_json::map;

use crate::aoc::Solution;

pub struct One;

#[derive(Debug)]
pub struct Calibration(u32);

impl From<&str> for Calibration {
    fn from(line: &str) -> Self {
        let digits = line.chars().filter(|c| c.is_digit(10)).collect_vec();

        let mut s = String::new();
        s.push(*digits.first().unwrap());
        s.push(*digits.last().unwrap());

        Calibration(s.parse().unwrap())
    }
}

impl Solution for One {
    type Output = u32;
    type Parsed = Vec<Calibration>;

    fn input() -> &'static str {
        include_str!("../inputs/1.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input
            .lines()
            .into_iter()
            .map(Calibration::from)
            .collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed.iter().map(|c| c.0).sum()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
