use std::ops::Add;

use itertools::Itertools;

use super::Solution;

pub struct DayOne;

impl Solution for DayOne {
    type Output = usize;

    fn input() -> String {
        include_str!("./inputs/1.txt").to_string()
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        // Split by line
        let items = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap());

        let increase_count = items
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count();

        increase_count
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        // Split by line
        let items = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap());

        let triple_increase_count = items
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count();

        triple_increase_count
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1527, 1575)
    }
}

pub struct DayTwo;

impl DayTwo {
    fn get_movements(input: String) -> Vec<(String, i32)> {
        // Split by line
        let movements = input.lines().filter(|s| !s.is_empty()).map(|s| {
            let mut splitter = s.split_ascii_whitespace();
            (
                splitter.next().unwrap().to_string(),
                splitter.next().unwrap().parse::<i32>().unwrap(),
            )
        });

        movements.collect()
    }
}

impl Solution for DayTwo {
    type Output = i32;

    fn input() -> String {
        include_str!("./inputs/2.txt").to_string()
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        let mut horizontal = 0;
        let mut depth = 0;

        for (instruction, movement) in Self::get_movements(input.to_string()) {
            match instruction.as_str() {
                "up" => depth -= movement,
                "down" => depth += movement,
                "forward" => horizontal += movement,
                _ => panic!("Undefined instruction"),
            };
        }

        horizontal * depth
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        for (instruction, movement) in Self::get_movements(input.to_string()) {
            match instruction.as_str() {
                "up" => aim -= movement,
                "down" => aim += movement,
                "forward" => {
                    horizontal += movement;
                    depth += aim * movement;
                }
                _ => panic!("Undefined instruction"),
            };
        }

        horizontal * depth
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1868935, 1965970888)
    }
}

pub struct DayThree;

impl Solution for DayThree {
    type Output = u64;

    fn input() -> String {
        include_str!("./inputs/3.txt").to_string()
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        let bins = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|c| c.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let bins_length = bins.len();
        let mut ones_count = vec![0; bins[0].len()];

        for bin in bins {
            for (i, bit) in bin.iter().enumerate() {
                if bit == &'1' {
                    (*ones_count.get_mut(i).unwrap()) += 1;
                }
            }
        }

        let gamma_rate = ones_count
            .iter()
            .map(|one_count| {
                if one_count >= &(bins_length / 2) {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>();

        let epsilon_rate = gamma_rate
            .clone()
            .chars()
            .map(|c| match c {
                '0' => '1',
                '1' => '0',
                _ => panic!("Wrong digit"),
            })
            .collect::<String>();

        let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
        let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();

        (gamma_rate * epsilon_rate) as u64
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        todo!()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (852500, 0)
    }
}

pub struct DayFour;
pub struct DayFive;
pub struct DaySix;
pub struct DaySeven;
pub struct DayEight;
pub struct DayNine;
pub struct DayTen;
pub struct DayEleven;
pub struct DayTwelve;
pub struct DayThirteen;
pub struct DayFourteen;
pub struct DayFifteen;
pub struct DaySixteen;
pub struct DaySeventeen;
pub struct DayEighteen;
pub struct DayNineteen;
pub struct DayTwenty;
pub struct DayTwentyOne;
pub struct DayTwentyTwo;
pub struct DayTwentyThree;
pub struct DayTwentyFour;
pub struct DayTwentyFive;
