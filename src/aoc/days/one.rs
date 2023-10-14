use itertools::Itertools;

use crate::aoc::Solution;

#[derive(Debug)]
pub enum Instruction {
    Up,
    Down,
}

pub struct One;

impl Solution for One {
    type Output = i32;
    type Parsed = Vec<Instruction>;

    fn input() -> &'static str {
        include_str!("../inputs/1.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input
            .chars()
            .map(|c| match c {
                '(' => Instruction::Up,
                ')' => Instruction::Down,
                _ => panic!("lol"),
            })
            .collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .iter()
            .map(|inst| match inst {
                Instruction::Up => 1,
                Instruction::Down => -1,
            })
            .sum()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut floor = 1;

        for (i, inst) in parsed.iter().enumerate() {
            match inst {
                Instruction::Up => floor += 1,
                Instruction::Down => floor -= 1,
            }

            if floor == -1 {
                return i as i32;
            }
        }

        panic!("No solution lol");
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (280, 1797)
    }
}
