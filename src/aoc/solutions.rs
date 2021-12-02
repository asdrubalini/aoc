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

impl Solution for DayTwo {
    type Output = i32;

    fn input() -> String {
        include_str!("./inputs/2.txt").to_string()
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        // Split by line
        let movements = input.lines().filter(|s| !s.is_empty()).map(|s| {
            let mut splitter = s.split_ascii_whitespace();
            (
                splitter.next().unwrap(),
                splitter.next().unwrap().parse::<i32>().unwrap(),
            )
        });

        let mut horizontal = 0;
        let mut depth = 0;

        for (instruction, movement) in movements {
            match instruction {
                "up" => depth -= movement,
                "down" => depth += movement,
                "forward" => horizontal += movement,
                _ => panic!("Undefined instruction"),
            };
        }

        horizontal * depth
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        todo!()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1868935, 0)
    }
}

pub struct DayThree;
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
