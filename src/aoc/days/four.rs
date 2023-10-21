use md5::{Digest, Md5};

use crate::aoc::Solution;

pub struct Four;

impl Solution for Four {
    type Output = u32;
    type Parsed = String;

    fn input() -> &'static str {
        include_str!("../inputs/4.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        String::from(input)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut nonce: u32 = 0;
        let starts_with = "00000";

        loop {
            let mut hasher = Md5::new();
            hasher.update(parsed);
            hasher.update(nonce.to_string());

            let result = hasher.finalize();
            let hash = base16ct::lower::encode_string(&result);

            if hash.starts_with(starts_with) {
                break;
            }

            nonce += 1;
        }

        nonce
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
