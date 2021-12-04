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

        items
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count()
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        // Split by line
        let items = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap());

        items
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(prev, next)| next > prev)
            .count()
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

impl DayThree {
    fn get_bins(input: String) -> Vec<Vec<char>> {
        input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|c| c.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>()
    }

    fn filter_out_bits(
        mut candidates: Vec<Vec<char>>,
        filter: fn(bit: char, one_count: usize, zero_count: usize) -> bool,
    ) -> u64 {
        let mut current_bit = 0;

        while candidates.len() > 1 {
            let one_bits_count = candidates
                .iter()
                .map(|bits| bits.get(current_bit).unwrap())
                .filter(|bit| bit == &&'1')
                .count();

            let zero_bits_count = candidates
                .iter()
                .map(|bits| bits.get(current_bit).unwrap())
                .filter(|bit| bit == &&'0')
                .count();

            candidates = candidates
                .iter_mut()
                .filter(|bits| {
                    let bit = bits.get(current_bit).unwrap();
                    filter(*bit, one_bits_count, zero_bits_count)
                })
                .map(|bits| bits.to_owned())
                .collect();

            current_bit += 1;
        }

        let rating = candidates.get_mut(0).unwrap().iter().collect::<String>();
        u64::from_str_radix(&rating, 2).unwrap()
    }
}

impl Solution for DayThree {
    type Output = u64;

    fn input() -> String {
        include_str!("./inputs/3.txt").to_string()
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();
        let bins = Self::get_bins(input.to_string());

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
            .chars()
            .map(|c| match c {
                '0' => '1',
                '1' => '0',
                _ => panic!("Wrong digit"),
            })
            .collect::<String>();

        let gamma_rate = u64::from_str_radix(&gamma_rate, 2).unwrap();
        let epsilon_rate = u64::from_str_radix(&epsilon_rate, 2).unwrap();

        gamma_rate * epsilon_rate
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();
        let bins = Self::get_bins(input.to_string());

        let oxygen_generator_rating =
            Self::filter_out_bits(bins.clone(), |bit, one_count, zero_count| {
                bit == if one_count >= zero_count { '1' } else { '0' }
            });

        let scrubber_rating = Self::filter_out_bits(bins, |bit, one_count, zero_count| {
            bit == if zero_count <= one_count { '0' } else { '1' }
        });

        oxygen_generator_rating * scrubber_rating
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (852500, 1007985)
    }
}

pub struct DayFour;

impl DayFour {
    fn parse_drawn_numbers(input: &str) -> Vec<u8> {
        input
            .lines()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect()
    }

    fn parse_boards(input: &str) -> Vec<Board> {
        let boards_raw = input.split("\n\n").skip(1);
        boards_raw.map(Board::new).collect()
    }
}

#[derive(Debug)]
struct Board {
    inner: Vec<Vec<u8>>,
}

impl Board {
    fn new(input: &str) -> Self {
        let inner = input
            .lines()
            .map(|line| {
                line.split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|row| !row.is_empty())
            .collect::<Vec<_>>();

        Self { inner }
    }

    fn check_win(&self, drawn_numbers: &[u8]) -> bool {
        // check horizontally
        for row in &self.inner {
            // check for match
            if row.iter().all(|elem| drawn_numbers.contains(elem)) {
                return true;
            }
        }

        // check vertically
        for i in 0..self.inner.len() {
            let mut col = self.inner.iter().map(|row| row.get(i).unwrap());
            // check for match
            if col.all(|elem| drawn_numbers.contains(elem)) {
                return true;
            }
        }

        false
    }

    fn winner_score(&self, drawn_numbers: &[u8]) -> u32 {
        assert!(self.check_win(drawn_numbers));

        let unmarked_sum: u32 = self
            .inner
            .iter()
            .flatten()
            .filter(|item| !drawn_numbers.contains(item))
            .map(|n| *n as u32)
            .sum();

        unmarked_sum * (*drawn_numbers.last().unwrap() as u32)
    }
}

impl Solution for DayFour {
    type Output = u32;

    fn input() -> String {
        include_str!("./inputs/4.txt").to_string()
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        let numbers = Self::parse_drawn_numbers(input);
        let boards = Self::parse_boards(input);

        let winner_score = (0..numbers.len())
            .map(|i| {
                let drawn_numbers = numbers.get(0..=i).unwrap();

                let winner = boards
                    .iter()
                    .filter(|b| b.check_win(&drawn_numbers))
                    .collect::<Vec<_>>();

                if winner.len() > 0 {
                    let winner = winner.get(0).unwrap();
                    Some(winner.winner_score(drawn_numbers))
                } else {
                    None
                }
            })
            .filter(|b| b.is_some())
            .map(|b| b.unwrap())
            .nth(0)
            .unwrap();

        winner_score
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        todo!()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (16674, 0)
    }
}

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
