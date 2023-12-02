use itertools::Itertools;

use crate::aoc::Solution;

pub struct One;

trait Calibration {
    fn calibrate_v1(line: &str) -> u32;
    fn calibrate_v2(line: &str) -> u32;
}

impl Calibration for u32 {
    fn calibrate_v1(line: &str) -> u32 {
        let digits = line.chars().filter(|c| c.is_digit(10)).collect_vec();

        dbg!(&digits);

        let mut s = String::new();
        s.push(*digits.first().unwrap());
        s.push(*digits.last().unwrap());

        s.parse().unwrap()
    }

    fn calibrate_v2(line: &str) -> u32 {
        let mut line = line.to_string();

        dbg!(&line);

        let replacements = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        for (i, r) in replacements.into_iter().enumerate() {
            let mut replacement = r.to_string();
            replacement.push_str(&(i + 1).to_string());
            replacement.push_str(r);

            line = line.replace(r, &replacement);
        }

        dbg!(&line);

        Self::calibrate_v1(&line)
    }
}

impl Solution for One {
    type Output = u32;
    type Parsed = Vec<&'static str>;

    fn input() -> &'static str {
        include_str!("../inputs/1.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().into_iter().collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed.into_iter().map(|l| u32::calibrate_v1(*l)).sum()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        parsed.into_iter().map(|l| u32::calibrate_v2(*l)).sum()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (55208, 54578)
    }
}
