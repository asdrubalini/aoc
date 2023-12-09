use crate::aoc::Solution;
use itertools::Itertools;

pub struct Six;

#[derive(Debug, Clone, Copy)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn can_beat_the_record(&self, hold_time: u64) -> bool {
        let speed = hold_time; // mm / s
        let moving_duration = self.time - hold_time; // s
        let travelled_distance = speed * moving_duration; // mm

        travelled_distance > self.distance
    }

    fn bruteforce_record_breaking_ways_count(&self) -> u64 {
        (0..=self.time)
            .filter(|hold_time| self.can_beat_the_record(*hold_time))
            .count() as u64
    }
}

impl From<(u64, u64)> for Race {
    fn from(value: (u64, u64)) -> Self {
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

impl SheetOfPaper {
    fn clone_into_v2(&self) -> Race {
        let time: String = self
            .races
            .iter()
            .map(|race| race.time.to_string())
            .collect();

        let distance: String = self
            .races
            .iter()
            .map(|race| race.distance.to_string())
            .collect();

        Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        }
    }
}

impl From<&str> for SheetOfPaper {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();

        let parse = |a: &str| -> Vec<u64> {
            a.split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect_vec()
        };

        let races = parse(lines.next().unwrap())
            .into_iter()
            .zip(parse(lines.next().unwrap()))
            .map(Into::into)
            .collect_vec();

        SheetOfPaper { races }
    }
}

impl Solution for Six {
    type Output = u64;
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

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let race = parsed.clone_into_v2();
        race.bruteforce_record_breaking_ways_count()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (861300, 28101347)
    }
}
