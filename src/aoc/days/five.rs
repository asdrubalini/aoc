use std::collections::VecDeque;

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Five;

#[derive(Debug, Clone, Copy)]
struct Crate(char);

impl From<char> for Crate {
    fn from(from: char) -> Self {
        Crate(from)
    }
}

#[derive(Debug, Clone)]
pub struct Stacks {
    stacks: Vec<VecDeque<Crate>>,
}

impl From<&str> for Stacks {
    fn from(from: &str) -> Self {
        let lines = from.lines().rev().collect_vec();
        let stacks_positions = lines
            .first()
            .unwrap()
            .chars()
            .enumerate()
            .filter_map(|(i, char)| if char.is_numeric() { Some(i) } else { None })
            .collect_vec();

        let stacks = stacks_positions
            .into_iter()
            .map(|pos| {
                let stacks: VecDeque<Crate> = lines
                    .iter()
                    .skip(1)
                    .map(|line| line.chars().nth(pos).unwrap())
                    .filter(|chr| chr.is_alphabetic())
                    .map(Crate::from)
                    .rev()
                    .collect();

                stacks
            })
            .collect_vec();

        Stacks { stacks }
    }
}

impl Stacks {
    fn execute_command(&mut self, command: Command) {
        for _ in 0..command.quantity {
            let stack_from = self.stacks.get_mut(command.from).unwrap();
            let moved_crate = stack_from.pop_front().unwrap();

            let stack_to = self.stacks.get_mut(command.to).unwrap();
            stack_to.push_front(moved_crate);
        }
    }

    fn get_top(&self) -> Vec<Crate> {
        self.stacks
            .iter()
            .map(|stack| stack.front().unwrap())
            .copied()
            .collect_vec()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Command {
    quantity: u32,
    from: usize,
    to: usize,
}

impl From<&str> for Command {
    fn from(from: &str) -> Self {
        // move 1 from 2 to 3
        let splitted = from.split(' ').collect_vec();

        let quantity = splitted[1].parse().unwrap();
        let from = splitted[3].parse::<usize>().unwrap() - 1; // make it an index
        let to = splitted[5].parse::<usize>().unwrap() - 1; // make it an index

        Command { quantity, from, to }
    }
}

impl Solution for Five {
    type Output = String;
    type Parsed = (Stacks, Vec<Command>);

    fn input() -> &'static str {
        include_str!("../inputs/5.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        let (stacks_config, commands) = input.split("\n\n").collect_tuple().unwrap();
        let stacks = Stacks::from(stacks_config);
        let commands = commands.lines().map(Command::from).collect_vec();

        (stacks, commands)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let (mut stacks, commands) = parsed.to_owned();

        for command in commands {
            stacks.execute_command(command);
        }

        String::from_iter(stacks.get_top().into_iter().map(|c| c.0))
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        "".to_string()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        ("MQSHJMWNH".to_string(), "".to_string())
    }
}
