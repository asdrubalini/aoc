use itertools::Itertools;

use crate::aoc::Solution;

pub struct Six;

fn find_first_unique_index(signal: &str, unique_len: usize) -> u32 {
    for (pos, bytes) in signal.as_bytes().windows(unique_len).enumerate() {
        if bytes.iter().unique().count() == unique_len {
            return (pos + unique_len) as u32;
        }
    }

    panic!("lol")
}

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
        find_first_unique_index(parsed, 4)
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        find_first_unique_index(parsed, 14)
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1625, 2250)
    }
}
