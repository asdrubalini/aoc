use crate::aoc::Solution;

trait LookAndSay {
    fn transform_sequence(&self) -> Self;
}

impl LookAndSay for String {
    fn transform_sequence(&self) -> String {
        let mut chars = self.chars().into_iter().peekable();
        let mut s = String::new();

        while let Some(curr) = chars.next() {
            let mut count = 1;

            while let Some(next) = chars.peek() {
                if curr != *next {
                    break;
                }

                chars.next();
                count += 1;
            }

            s.push_str(&format!("{count}{curr}"));
        }

        s
    }
}

pub struct Ten;

impl Solution for Ten {
    type Output = u32;
    type Parsed = String;

    fn input() -> &'static str {
        include_str!("../inputs/10.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        String::from(input)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut s = parsed.clone();

        for _ in 0..40 {
            s = s.transform_sequence();
        }

        s.len() as u32
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut s = parsed.clone();

        for _ in 0..50 {
            s = s.transform_sequence();
        }

        s.len() as u32
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (252594, 3579328)
    }
}
