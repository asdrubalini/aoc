use std::collections::HashMap;

use itertools::Itertools;

use crate::aoc::Solution;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN_SEQUENCES: [&str; 4] = ["ab", "cd", "pq", "xy"];

trait NiceString {
    fn is_nice_v1(&self) -> bool;
    fn is_nice_v2(&self) -> bool;
}

impl NiceString for String {
    fn is_nice_v1(&self) -> bool {
        let chars = self.chars().collect::<Vec<char>>();

        // at least 3 vowels
        if chars.iter().filter(|chr| VOWELS.contains(chr)).count() < 3 {
            return false;
        }

        // at least one letter that appears twice in a row
        if chars.iter().tuple_windows().filter(|(a, b)| a == b).count() < 1 {
            return false;
        }

        // does not contain forbidden sequences
        if chars
            .windows(2)
            .filter(|seq| {
                let seq: String = seq.iter().collect();
                FORBIDDEN_SEQUENCES.contains(&seq.as_str())
            })
            .count()
            > 0
        {
            return false;
        }

        true
    }

    fn is_nice_v2(&self) -> bool {
        let chars = self.chars().collect::<Vec<char>>();

        // it contains a pair of any two letters that appears at least twice in the string without overlapping
        let mut pairs_count: HashMap<String, u16> = HashMap::default();
        let mut iter = chars.windows(2).peekable();

        while let Some(seq) = iter.next() {
            let curr: String = seq.iter().collect();
            let next = iter.peek().map(|s| s.iter().collect::<String>());

            *pairs_count.entry(curr.clone()).or_default() += 1;

            if let Some(next) = next {
                if next == curr {
                    iter.next();
                }
            }
        }

        if pairs_count.values().filter(|c| **c >= 2).count() == 0 {
            return false;
        }

        // it contains at least one letter which repeats with exactly one letter between them
        if chars
            .iter()
            .tuple_windows()
            .filter(|(a, _b, c)| a == c)
            .count()
            == 0
        {
            return false;
        }

        true
    }
}

pub struct Five;

impl Solution for Five {
    type Output = u32;
    type Parsed = Vec<String>;

    fn input() -> &'static str {
        include_str!("../inputs/5.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(String::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed.iter().filter(|s| s.is_nice_v1()).count() as u32
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        parsed.iter().filter(|s| s.is_nice_v2()).count() as u32
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (238, 0)
    }
}
