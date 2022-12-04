use itertools::Itertools;

use crate::aoc::Solution;

pub struct Four;

#[derive(Debug)]
pub struct SectionRange {
    start: u32,
    stop: u32,
}

impl SectionRange {
    fn len(&self) -> u32 {
        self.stop.abs_diff(self.start)
    }

    fn fully_include_other(&self, other: &Self) -> bool {
        for i in other.start..=other.stop {
            if i < self.start || i > self.stop {
                return false;
            }
        }

        return true;
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
            .map(|(first, second)| {
                if first.len() > second.len() {
                    first.fully_include_other(second) as u32
                } else if second.len() > first.len() {
                    second.fully_include_other(first) as u32
                } else {
                    (first.fully_include_other(second) || second.fully_include_other(first)) as u32
                }
            })
            .sum::<u32>()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (498, 0)
    }
}
