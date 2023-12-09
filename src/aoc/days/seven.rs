use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Seven;

/// Five of a kind, where all five cards have the same label: AAAAA
/// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
/// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
/// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
/// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
/// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
/// High card, where all cards' labels are distinct: 23456
#[derive(Debug, Clone, Copy)]
pub enum HandKind {
    FiveOfAKind,  // len = 1
    FourOfAKind,  // len = 2
    FullHouse,    // len = 2
    ThreeOfAKind, // len = 3
    TwoPair,      // len = 3
    OnePair,      // len = 4
    HighCard,     // len = 5
}

impl Into<u32> for HandKind {
    fn into(self) -> u32 {
        match self {
            HandKind::FiveOfAKind => 7,
            HandKind::FourOfAKind => 6,
            HandKind::FullHouse => 5,
            HandKind::ThreeOfAKind => 4,
            HandKind::TwoPair => 3,
            HandKind::OnePair => 2,
            HandKind::HighCard => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Card(char);

impl From<char> for Card {
    fn from(chr: char) -> Self {
        Card(chr)
    }
}

impl Into<u32> for Card {
    fn into(self) -> u32 {
        match self.0 {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            _ => panic!("unexpected"),
        }
    }
}

#[derive(Debug, Clone, Eq, Ord)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Equal))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a: u32 = self.kind().into();
        let b: u32 = other.kind().into();

        match a.partial_cmp(&b) {
            Some(Ordering::Equal) => {
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    let c1: u32 = (*c1).into();
                    let c2: u32 = (*c2).into();

                    match c1.partial_cmp(&c2) {
                        Some(Ordering::Equal) => continue,
                        ord => return ord,
                    }
                }

                return Some(Ordering::Equal);
            }
            ord => return ord,
        }
    }
}

impl Hand {
    fn kind(&self) -> HandKind {
        let mut count: HashMap<Card, u32> = HashMap::new();

        for card in self.cards.iter() {
            *count.entry(*card).or_default() += 1;
        }

        match count.len() {
            1 => HandKind::FiveOfAKind,

            2 => match *count.values().sorted().max().unwrap() {
                4 => HandKind::FourOfAKind,
                3 => HandKind::FullHouse,
                _ => panic!("unexpected"),
            },

            3 => match *count.values().sorted().max().unwrap() {
                3 => HandKind::ThreeOfAKind,
                2 => HandKind::TwoPair,
                _ => panic!("unexpected"),
            },

            4 => HandKind::OnePair,
            5 => HandKind::HighCard,
            _ => panic!("unexpected"),
        }
    }
}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let (cards, idk) = line.split_ascii_whitespace().collect_tuple().unwrap();

        let cards = cards.chars().map(Card::from).collect_vec();
        let idk = idk.parse().unwrap();

        Hand { cards, bid: idk }
    }
}

impl Solution for Seven {
    type Output = u32;
    type Parsed = Vec<Hand>;

    fn input() -> &'static str {
        include_str!("../inputs/7.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Hand::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut ordered = parsed.to_owned();
        ordered.sort();

        ordered
            .into_iter()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
            .sum()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (245794640, 0)
    }
}
