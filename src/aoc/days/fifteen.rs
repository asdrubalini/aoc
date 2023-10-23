use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::aoc::Solution;

pub struct Fifteen;

lazy_static! {
    static ref TAPE: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();

        m.insert("children", 3);
        m.insert("cats", 7);
        m.insert("samoyeds", 2);
        m.insert("pomeranians", 3);
        m.insert("akitas", 0);
        m.insert("vizslas", 0);
        m.insert("goldfish", 5);
        m.insert("trees", 3);
        m.insert("cars", 2);
        m.insert("perfumes", 1);

        m
    };
}

#[derive(Debug, Clone)]
pub struct Aunt {
    id: u32,
    objects: HashMap<String, u32>,
}

impl From<&str> for Aunt {
    fn from(line: &str) -> Self {
        let tokens = line.split(' ').collect_vec();

        let id = tokens.get(1).unwrap().replace(':', "").parse().unwrap();
        let mut info = HashMap::new();

        let tokens = tokens.into_iter().skip(2).collect_vec();
        let count = tokens.len() / 2;

        for i in 0..count {
            let name = tokens.get(i * 2).unwrap().replace(':', "").parse().unwrap();
            let value = tokens
                .get(i * 2 + 1)
                .unwrap()
                .replace(',', "")
                .parse()
                .unwrap();

            info.insert(name, value);
        }

        Aunt { id, objects: info }
    }
}

impl Solution for Fifteen {
    type Output = u32;
    type Parsed = Vec<Aunt>;

    fn input() -> &'static str {
        include_str!("../inputs/15.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Aunt::from).collect_vec()
    }

    fn solve_first(aunts: &Self::Parsed) -> Self::Output {
        for aunt in aunts {
            let matching_count = aunt
                .objects
                .iter()
                .filter(|(object, value)| {
                    if let Some(real_value) = TAPE.get(object.as_str()) {
                        if *real_value == **value {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .count();

            if matching_count == aunt.objects.len() {
                return aunt.id;
            }
        }

        unreachable!();
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (373, 0)
    }
}
