use md5::{Digest, Md5};

use crate::aoc::Solution;

pub struct Four;

impl Four {
    fn mine(secret_key: impl AsRef<str>, required_prefix: impl AsRef<str>) -> u32 {
        let mut nonce: u32 = 0;

        loop {
            let mut hasher = Md5::new();
            hasher.update(secret_key.as_ref());
            hasher.update(nonce.to_string());

            let result = hasher.finalize();
            let hash = base16ct::lower::encode_string(&result);

            if hash.starts_with(required_prefix.as_ref()) {
                break;
            }

            nonce += 1;
        }

        nonce
    }
}

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
        Self::mine(parsed, "00000")
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        Self::mine(parsed, "000000")
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (346386, 9958218)
    }
}
