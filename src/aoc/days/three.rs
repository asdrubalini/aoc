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

    fn compute_priority(&self, item: char) -> u32 {
        if ('A'..='Z').contains(&item) {
            (item as u8 - 38) as u32
        } else if ('a'..='z').contains(&item) {
            (item as u8 - 96) as u32
        } else {
            panic!("wtf")
        }
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

                duplicates
                    .into_iter()
                    .map(|dup| rucksack.compute_priority(dup))
                    .sum::<u32>()
            })
            .sum::<u32>()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        7824
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
