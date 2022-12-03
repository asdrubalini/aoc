use std::collections::HashMap;

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Three;

#[derive(Debug, Clone)]
pub struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
}

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        let (left, right) = input.split_at(input.len() / 2);

        Self {
            left: left.chars().collect_vec(),
            right: right.chars().collect_vec(),
        }
    }
}

impl Rucksack {
    fn find_duplicates(&self) -> Vec<char> {
        let mut duplicates = vec![];

        for item in self.left.iter() {
            if self.right.contains(item) && !duplicates.contains(item) {
                duplicates.push(*item);
            }
        }

        duplicates
    }

    fn compute_priority(item: char) -> u32 {
        if ('A'..='Z').contains(&item) {
            (item as u8 - 38) as u32
        } else if ('a'..='z').contains(&item) {
            (item as u8 - 96) as u32
        } else {
            panic!("wtf")
        }
    }

    fn unique_badges(&self) -> Vec<char> {
        self.left
            .iter()
            .merge(self.right.iter())
            .unique()
            .map(|c| c.to_owned())
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
                    .map(|dup| Rucksack::compute_priority(dup))
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
                    .map(|c| *c)
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
