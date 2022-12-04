use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Three;

#[derive(Debug, Clone)]
pub struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        let (left, right) = input.split_at(input.len() / 2);

        Self {
            left: HashSet::from_iter(left.chars()),
            right: HashSet::from_iter(right.chars()),
        }
    }
}

impl Rucksack {
    /// find badges contained in both left and right
    fn find_duplicates(&self) -> Vec<char> {
        self.left
            .iter()
            .filter_map(|item| {
                if self.right.contains(item) {
                    Some(*item)
                } else {
                    None
                }
            })
            .unique()
            .collect()
    }

    fn compute_priority(item: char) -> u32 {
        match item {
            'A'..='Z' => (item as u8 - 38) as u32, // 1 to 26
            'a'..='z' => (item as u8 - 96) as u32, // 26 to 52
            _ => panic!("wtf"),
        }
    }

    fn unique_badges(&self) -> Vec<char> {
        self.left
            .iter()
            .merge(self.right.iter())
            .unique()
            .copied()
            .collect()
    }
}

impl Solution for Three {
    type Output = u32;
    type Parsed = Vec<Rucksack>;

    fn input() -> &'static str {
        include_str!("../inputs/3.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Rucksack::from).collect()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .iter()
            .map(|rucksack| {
                let duplicates = rucksack.find_duplicates();

                // find duplicate badges and sum the priorities
                duplicates
                    .into_iter()
                    .map(Rucksack::compute_priority)
                    .sum::<u32>()
            })
            .sum::<u32>()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let group_badges: Vec<char> = parsed
            .iter()
            .chunks(3)
            .into_iter()
            .map(|rucksucks| {
                let mut badge_counter: HashMap<char, u8> = HashMap::new();

                // count how many times each badge appears
                for sack in rucksucks.map(|r| r.unique_badges()) {
                    for badge in sack {
                        *badge_counter.entry(badge).or_default() += 1;
                    }
                }

                // there should be just one badge with counter of three
                let badge: Vec<char> = badge_counter
                    .iter()
                    .filter_map(|(key, value)| if *value == 3 { Some(key) } else { None })
                    .copied()
                    .collect();

                assert_eq!(badge.len(), 1);

                badge.first().unwrap().to_owned()
            })
            .collect();

        group_badges
            .iter()
            .map(|c| Rucksack::compute_priority(*c))
            .sum::<u32>()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (7824, 2798)
    }
}
