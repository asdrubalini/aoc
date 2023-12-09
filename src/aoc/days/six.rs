use crate::aoc::Solution;
use itertools::Itertools;

pub struct Six;

#[derive(Debug)]
pub struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn can_beat_the_record(&self, hold_time: u32) -> bool {
        let speed = hold_time; // mm / s
        let moving_duration = self.time - hold_time; // s
        let travelled_distance = speed * moving_duration; // m

        travelled_distance > self.distance
    }

    fn bruteforce_record_breaking_ways_count(&self) -> u32 {
        (0..=self.time)
            .into_iter()
            .filter(|hold_time| self.can_beat_the_record(*hold_time))
            .count() as u32
    }
}

impl From<(u32, u32)> for Race {
    fn from(value: (u32, u32)) -> Self {
        Race {
            time: value.0,
            distance: value.1,
        }
    }
}

#[derive(Debug)]
pub struct SheetOfPaper {
    races: Vec<Race>,
}

impl From<&str> for SheetOfPaper {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();

        let parse = |a: &str| -> Vec<u32> {
            a.split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect_vec()
        };

        let races = parse(lines.next().unwrap())
            .into_iter()
            .zip(parse(lines.next().unwrap()).into_iter())
            .map(Into::into)
            .collect_vec();

        SheetOfPaper { races }
    }
}

impl Solution for Six {
    type Output = u32;
    type Parsed = SheetOfPaper;

    fn input() -> &'static str {
        include_str!("../inputs/6.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        SheetOfPaper::from(input)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .races
            .iter()
            .map(|race| race.bruteforce_record_breaking_ways_count())
            .product()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (861300, 0)
    }
}
