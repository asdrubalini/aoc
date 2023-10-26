use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::aoc::Solution;

pub struct Sixteen;

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

impl Solution for Sixteen {
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
            let is_current_aunt = aunt.objects.iter().all(|(object, aunt_value)| {
                if let Some(real_value) = TAPE.get(object.as_str()) {
                    real_value == aunt_value
                } else {
                    false
                }
            });

            if is_current_aunt {
                return aunt.id;
            }
        }

        unreachable!();
    }

    fn solve_second(aunts: &Self::Parsed) -> Self::Output {
        for aunt in aunts {
            let is_current_aunt = aunt.objects.iter().all(|(object, aunt_value)| {
                // the cats and trees readings indicates that there are greater than that many
                // the pomeranians and goldfish readings indicate that there are fewer than that many

                if let Some(real_value) = TAPE.get(object.as_str()) {
                    if object == "cats" || object == "trees" {
                        aunt_value > real_value
                    } else if object == "pomeranians" || object == "goldfish" {
                        aunt_value < real_value
                    } else {
                        real_value == aunt_value
                    }
                } else {
                    false
                }
            });

            if is_current_aunt {
                return aunt.id;
            }
        }

        unreachable!();
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (373, 260)
    }
}
