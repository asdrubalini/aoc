use std::collections::VecDeque;

use itertools::Itertools;

use crate::aoc::Solution;

trait AocEncoded {
    fn escape(s: &str) -> String;
    fn len_in_memory(&self) -> usize;
}

impl AocEncoded for String {
    fn escape(s: &str) -> String {
        let chars = s.chars().peekable();

        let mut escaped_chars = VecDeque::new();

        for chr in chars {
            match chr {
                '\\' => {
                    escaped_chars.push_back("\\\\".to_string());
                }

                '\"' => {
                    escaped_chars.push_back("\\\"".to_string());
                }

                _ => {
                    escaped_chars.push_back(chr.to_string());
                }
            }
        }

        // append and prepend the quotation marks
        escaped_chars.push_front("\"".to_string());
        escaped_chars.push_back("\"".to_string());

        todo!()
    }

    /// Encoded string lenght in memory (without escaping)
    fn len_in_memory(&self) -> usize {
        let mut chars = self.chars().collect_vec();
        assert_eq!(*chars.first().unwrap(), '"');
        assert_eq!(*chars.last().unwrap(), '"');

        chars.remove(0);
        chars.pop();

        let mut chars = chars.into_iter().peekable();
        let mut len = 0;

        while let Some(chr) = chars.next() {
            match chr {
                '\\' => {
                    let next = chars.peek().unwrap();

                    let to_skip = match *next {
                        '"' | '\\' => 1,
                        'x' => 3,
                        _ => panic!("lol"),
                    };

                    chars.nth(to_skip - 1);
                    len += 1;
                }
                _ => len += 1,
            }
        }

        len
    }
}

pub struct Eight;

impl Solution for Eight {
    type Output = u32;
    type Parsed = Vec<String>;

    fn input() -> &'static str {
        include_str!("../inputs/8.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(String::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let original_len = parsed.iter().map(|s| s.len()).sum::<usize>() as u32;
        let in_memory_len = parsed.iter().map(|s| s.len_in_memory()).sum::<usize>() as u32;

        original_len - in_memory_len
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let original_len = parsed.iter().map(|s| s.len()).sum::<usize>() as u32;
        let reescaped_len = parsed
            .iter()
            .map(|s| String::escape(s).len())
            .sum::<usize>() as u32;

        reescaped_len - original_len
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1350, 0)
    }
}
