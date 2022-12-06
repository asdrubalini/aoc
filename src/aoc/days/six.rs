use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
    time::Instant,
};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Six;

/// faster than `.unique().count() == len` since this exists
/// as soon as a duplicated item is encountered
/// not really necessary but day six was boring as hell
fn all_unique<T: Hash + Eq + Ord + Copy>(items: &[T]) -> bool {
    let mut seen: BTreeSet<T> = BTreeSet::new();

    for item in items {
        if seen.contains(item) {
            return false;
        }

        seen.insert(*item);
    }

    true
}

fn find_first_unique_index(signal: &str, unique_len: usize) -> u32 {
    //let start = Instant::now();

    for (pos, bytes) in signal.as_bytes().windows(unique_len).enumerate() {
        if all_unique(bytes) {
            return (pos + unique_len) as u32;
        }
    }

    panic!("lol")
}

impl Solution for Six {
    type Output = u32;
    type Parsed = String;

    fn input() -> &'static str {
        include_str!("../inputs/6.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.to_string()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        find_first_unique_index(parsed, 4)
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        find_first_unique_index(parsed, 14)
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1625, 2250)
    }
}
