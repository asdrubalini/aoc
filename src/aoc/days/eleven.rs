use std::ops::AddAssign;

use itertools::Itertools;

use crate::aoc::Solution;

#[derive(Debug)]
struct Password(String);

impl Password {
    fn is_valid(&self) -> bool {
        // Passwords may not contain the letters i, o, or l, as these letters can be mistaken
        // for other characters and are therefore confusing.
        let invalid = ['i', 'o', 'l'];
        if self.0.chars().filter(|chr| invalid.contains(chr)).count() > 1 {
            return false;
        }

        // Passwords must include one increasing straight of at least three letters, like abc,
        // bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
        if self
            .0
            .chars()
            .tuple_windows()
            .filter(|(a, b, c)| {
                let a = *a as u8;
                let b = *b as u8 - 1;
                let c = *c as u8 - 2;

                a == b && b == c
            })
            .count()
            == 0
        {
            return false;
        }

        // Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
        let mut pairs = self.0.chars().tuple_windows().peekable();
        let mut pairs_count = 0;

        while let Some((a, b)) = pairs.next() {
            if a != b {
                continue;
            }

            pairs_count += 1;

            if let Some((next_a, next_b)) = pairs.peek() {
                if a == *next_a && a == *next_b {
                    pairs.next().unwrap();
                }
            }
        }

        if pairs_count < 2 {
            return false;
        }

        true
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Password(value)
    }
}

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

impl AddAssign<u32> for Password {
    fn add_assign(&mut self, rhs: u32) {
        let mut chars: Vec<char> = self.0.chars().collect();

        fn next_char(chr: char) -> char {
            if chr == 'z' {
                'a'
            } else {
                (chr as u8 + 1) as char
            }
        }

        for _ in 0..rhs {
            let mut curr_idx = chars.len() - 1;

            while curr_idx >= 0 {
                let curr_chr = chars.get_mut(curr_idx).unwrap();
                *curr_chr = next_char(*curr_chr);

                if *curr_chr != 'a' {
                    break;
                } else {
                    curr_idx -= 1;
                }
            }
        }

        self.0 = chars.into_iter().collect();
    }
}

pub struct Eleven;

impl Solution for Eleven {
    type Output = String;
    type Parsed = String;

    fn input() -> &'static str {
        include_str!("../inputs/11.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        String::from(input)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut password = Password::from(parsed.to_owned());

        while !password.is_valid() {
            password += 1;
        }

        password.into()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut password = Password::from(parsed.to_owned());

        while !password.is_valid() {
            password += 1;
        }

        password += 1;

        while !password.is_valid() {
            password += 1;
        }

        password.into()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        ("cqjxxyzz".to_string(), "cqkaabcc".to_string())
    }
}
