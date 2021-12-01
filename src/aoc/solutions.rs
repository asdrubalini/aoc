use itertools::Itertools;

use super::Solution;

pub struct DayOne;
pub struct DayTwo;
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

impl Solution for DayOne {
    type Output = (usize, usize);

    fn input() -> String {
        include_str!("./inputs/1.txt").to_string()
    }

    fn solve<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        // Split by line
        let items = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap());

        let increase_count = items
            .clone()
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count();

        let triple_increase_count = items
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count();

        (increase_count, triple_increase_count)
    }

    fn expected_solutions() -> Self::Output {
        (1527, 1575)
    }
}
