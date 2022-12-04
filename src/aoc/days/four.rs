use itertools::Itertools;

use crate::aoc::Solution;

pub struct Four;

#[derive(Debug)]
pub struct SectionRange {
    start: u32,
    stop: u32,
}

impl SectionRange {
    /// range len
    fn len(&self) -> u32 {
        self.stop.abs_diff(self.start)
    }

    /// if self fully includes other range
    fn fully_include_other(&self, other: &Self) -> bool {
        for i in other.start..=other.stop {
            if i < self.start || i > self.stop {
                return false;
            }
        }

        true
    }

    /// if the two ranges overlapping at least once
    fn overlap_with_other(&self, other: &Self) -> bool {
        for i in self.start..=self.stop {
            for j in other.start..=other.stop {
                if i == j {
                    return true;
                }
            }
        }

        false
    }
}

impl From<&str> for SectionRange {
    fn from(from: &str) -> Self {
        let (start, stop) = from.split('-').collect_tuple().unwrap();

        SectionRange {
            start: start.parse().unwrap(),
            stop: stop.parse().unwrap(),
        }
    }
}

impl Solution for Four {
    type Output = u32;
    type Parsed = Vec<(SectionRange, SectionRange)>;

    fn input() -> &'static str {
        include_str!("../inputs/4.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (first, second) = line.split(',').collect_tuple().unwrap();
                (SectionRange::from(first), SectionRange::from(second))
            })
            .collect()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .iter()
            .map(|(first, second)| match first.len().cmp(&second.len()) {
                std::cmp::Ordering::Less => second.fully_include_other(first),
                std::cmp::Ordering::Equal => {
                    first.fully_include_other(second) || second.fully_include_other(first)
                }
                std::cmp::Ordering::Greater => first.fully_include_other(second),
            })
            .filter(|b| *b)
            .count() as u32
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .iter()
            .map(|(first, second)| first.overlap_with_other(second))
            .filter(|b| *b)
            .count() as u32
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (498, 859)
    }
}
