use std::vec;

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Ten;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Self {
        let mut split = instruction.split_ascii_whitespace();

        match split.next().unwrap() {
            "addx" => {
                let increase = split.next().unwrap().parse().unwrap();
                Instruction::Addx(increase)
            }
            "noop" => Instruction::Noop,
            _ => panic!("wtf"),
        }
    }
}

impl IntoIterator for Instruction {
    type Item = Instruction;
    type IntoIter = vec::IntoIter<Instruction>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Instruction::Noop => vec![Instruction::Noop],
            Instruction::Addx(increase) => {
                vec![Instruction::Noop, Instruction::Addx(increase)]
            }
        }
        .into_iter()
    }
}

impl Solution for Ten {
    type Output = i32;
    type Parsed = Vec<Instruction>;

    fn input() -> &'static str {
        include_str!("../inputs/10.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        let instructions = input.lines().map(Instruction::from).collect_vec();

        instructions
            .into_iter()
            .flat_map(|a| a.into_iter())
            .collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut reg_x = 1;
        let mut values = vec![reg_x];

        for instruction in parsed {
            if let Instruction::Addx(increase) = instruction {
                reg_x += increase;
            }

            values.push(reg_x);
        }

        [20, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|i| values[i - 1] * i as i32)
            .sum::<i32>()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
