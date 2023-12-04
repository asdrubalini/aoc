use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Four;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let (id, other) = line.split(':').collect_tuple().unwrap();
        let id = id.replace(' ', "").replace("Card", "").parse().unwrap();

        let parse_numbers = |s: &str| -> HashSet<u32> {
            s.split_ascii_whitespace()
                .map(|number_str| number_str.parse().unwrap())
                .collect()
        };

        let (winning_numbers, my_numbers) =
            other.split('|').map(parse_numbers).collect_tuple().unwrap();

        Card {
            id,
            winning_numbers,
            my_numbers,
        }
    }
}

impl Solution for Four {
    type Output = u32;
    type Parsed = HashMap<u32, Card>;

    fn input() -> &'static str {
        include_str!("../inputs/4.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input
            .lines()
            .map(Card::from)
            .map(|card| (card.id, card))
            .collect()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .values()
            .map(|card| {
                let matching_count = card
                    .my_numbers
                    .iter()
                    .filter(|n| card.winning_numbers.contains(n))
                    .count() as u32;

                if matching_count > 0 {
                    2u32.pow(matching_count - 1)
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (20407, 0)
    }
}
