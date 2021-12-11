use itertools::Itertools;

use super::Solution;

pub struct DayOne;

enum Instruction {
    Up,
    Down,
}

impl DayOne {
    fn parse_input(input: &str) -> Vec<Instruction> {
        input
            .chars()
            .map(|chr| match chr {
                '(' => Instruction::Up,
                ')' => Instruction::Down,
                _ => panic!("Unknown instruction"),
            })
            .collect()
    }
}

impl Solution for DayOne {
    type Output = i64;

    fn input() -> &'static str {
        include_str!("./inputs/1.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let instructions = Self::parse_input(input);

        instructions
            .into_iter()
            .fold(0, |floor, instruction| match instruction {
                Instruction::Up => floor + 1,
                Instruction::Down => floor - 1,
            })
    }

    fn solve_second(input: &str) -> Self::Output {
        let instructions = Self::parse_input(input);
        let mut floor = 0;

        for (position, instruction) in instructions.into_iter().enumerate() {
            floor += match instruction {
                Instruction::Up => 1,
                Instruction::Down => -1,
            };

            if floor == -1 {
                return (position + 1) as i64;
            }
        }

        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (280, 1797)
    }
}
