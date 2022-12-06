use itertools::Itertools;

use crate::aoc::Solution;

pub struct Six;

impl Solution for Six {
    type Output = u32;
    type Parsed = String;

    fn input() -> &'static str {
        include_str!("../inputs/6.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.to_string()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        for (pos, (c1, c2, c3, c4)) in parsed.chars().tuple_windows().enumerate() {
            if vec![c1, c2, c3, c4].iter().unique().count() == 4 {
                return pos as u32 + 4;
            }
        }

        panic!("cannot find")
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
