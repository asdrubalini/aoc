use itertools::Itertools;

use crate::aoc::Solution;

trait ParsedLen {
    fn parsed_len(&self) -> usize;
}

impl ParsedLen for String {
    fn parsed_len(&self) -> usize {
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
                        _ => panic!("lol",),
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
        let total_len = parsed.iter().map(|s| s.len()).sum::<usize>() as u32;
        let parsed_len = parsed.iter().map(|s| s.parsed_len()).sum::<usize>() as u32;

        total_len - parsed_len
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1350, 0)
    }
}
