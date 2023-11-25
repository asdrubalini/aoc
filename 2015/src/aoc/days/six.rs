use core::panic;

use itertools::Itertools;

use crate::aoc::{
    utils::{Coord, InfiniteMatrix},
    Solution,
};

#[derive(Debug)]
pub enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
pub struct Instruction {
    action: Action,
    from: Coord,
    through: Coord,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let tokens = value.split(' ').collect_vec();

        let action = match *tokens.first().unwrap() {
            "turn" => match *tokens.get(1).unwrap() {
                "on" => Action::On,
                "off" => Action::Off,
                _ => panic!("lol"),
            },
            "toggle" => Action::Toggle,
            _ => panic!("lol"),
        };

        let from = *match action {
            Action::On | Action::Off => tokens.get(2).unwrap(),
            Action::Toggle => tokens.get(1).unwrap(),
        };

        let through = *match action {
            Action::On | Action::Off => tokens.get(4).unwrap(),
            Action::Toggle => tokens.get(3).unwrap(),
        };

        Instruction {
            action,
            from: from.into(),
            through: through.into(),
        }
    }
}

pub struct Six;

impl Solution for Six {
    type Output = u32;
    type Parsed = Vec<Instruction>;

    fn input() -> &'static str {
        include_str!("../inputs/6.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Instruction::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut m = InfiniteMatrix::<bool>::new_fixed(1000, 1000);

        for instruction in parsed {
            for y in instruction.from.y()..=instruction.through.y() {
                for x in instruction.from.x()..=instruction.through.x() {
                    let coord = Coord(x, y);
                    let entry = m.entry(coord);

                    match instruction.action {
                        Action::On => *entry.or_default() = true,
                        Action::Off => *entry.or_default() = false,
                        Action::Toggle => {
                            let status = entry.or_default();
                            *status = !*status;
                        }
                    }
                }
            }
        }

        // Count how many lights are lit
        m.into_iter().filter(|(_, status)| *status).count() as u32
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut m = InfiniteMatrix::<u16>::new_fixed(1000, 1000);

        for instruction in parsed {
            for y in instruction.from.y()..=instruction.through.y() {
                for x in instruction.from.x()..=instruction.through.x() {
                    let coord = Coord(x, y);
                    let entry = m.entry(coord);

                    match instruction.action {
                        Action::On => *entry.or_default() += 1,
                        Action::Off => {
                            let light = entry.or_default();
                            *light = light.saturating_sub(1);
                        }
                        Action::Toggle => *entry.or_default() += 2,
                    }
                }
            }
        }

        // Count how many lights are lit
        m.into_iter().map(|(_, status)| status as u32).sum::<u32>()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (569999, 0)
    }
}
