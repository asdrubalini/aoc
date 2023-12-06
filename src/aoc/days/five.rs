use std::{collections::HashMap, thread::current};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Five;

#[derive(Debug)]
pub struct Map {
    source_cat: String,
    dest_cat: String,

    mappings: Vec<(u64, u64, u64)>,
}

impl Map {
    fn translate(&self, n: u64) -> u64 {
        for (dest, source, len) in self.mappings.iter().copied() {
            if n >= source && n < source + len {
                return n - source + dest;
            }
        }

        n
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        println!("parsing {input}");

        let mut lines = input.lines();

        let binding = lines.next().unwrap().replace(" map:", "");
        let cats = binding.split('-').collect_vec();

        let (source_cat, dest_cat) = (cats[0].to_owned(), cats[2].to_owned());

        let mappings = lines
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect_tuple::<(u64, u64, u64)>()
                    .unwrap()
            })
            .collect();

        Map {
            source_cat,
            dest_cat,
            mappings,
        }
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<(String, String), Map>,
}

impl Almanac {
    fn translate(&self, source_cat: &str, dest_cat: &str, n: u64) -> u64 {
        let result = self
            .maps
            .get(&(source_cat.to_owned(), dest_cat.to_owned()))
            .unwrap()
            .translate(n);

        println!("{source_cat} -> {dest_cat}, {n} translated in {result}");

        result
    }

    fn find_next_target(&self, source_cat: &str) -> Option<String> {
        let ((_source, dest), _) = self.maps.iter().find(|((s, _), _)| s == source_cat)?;

        Some(dest.to_owned())
    }
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();

        let seeds = lines
            .next()
            .unwrap()
            .replace("seeds: ", "")
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_vec();

        lines.next();

        let binding = lines.join("\n");
        let blocks = binding.split("\n\n");

        let maps = blocks
            .into_iter()
            .map(Map::from)
            .map(|map| ((map.source_cat.to_owned(), map.dest_cat.to_owned()), map))
            .collect();

        Self { seeds, maps }
    }
}

impl Solution for Five {
    type Output = u32;
    type Parsed = Almanac;

    fn input() -> &'static str {
        include_str!("../inputs/5.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        Almanac::from(input)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut source_cat = "seed".to_string();
        let mut current_queue = parsed.seeds.clone();

        while let Some(target_cat) = parsed.find_next_target(&source_cat) {
            let next_queue = current_queue
                .iter()
                .map(|n| parsed.translate(&source_cat, &target_cat, *n))
                .collect_vec();

            current_queue = next_queue;
            source_cat = target_cat;
        }

        current_queue.into_iter().min().unwrap() as u32
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
