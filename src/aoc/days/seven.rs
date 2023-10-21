use core::panic;
use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

use crate::aoc::Solution;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Wire {
    Constant(u16),
    Named(String),
}

impl From<&&str> for Wire {
    fn from(name: &&str) -> Self {
        match name.parse::<u16>() {
            Ok(constant) => Wire::Constant(constant),
            Err(_) => Wire::Named(name.to_string()),
        }
    }
}

impl Wire {
    fn get_number(&self, cpu: &CPU) -> u16 {
        match self {
            Wire::Constant(constant) => *constant,
            Wire::Named(_) => cpu.get(self),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Load { dst: Wire, value: Wire },
    And { src: (Wire, Wire), dst: Wire },
    Or { src: (Wire, Wire), dst: Wire },
    Not { src: Wire, dst: Wire },
    LShift { src: (Wire, Wire), dst: Wire },
    RShift { src: (Wire, Wire), dst: Wire },
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let tokens = value.split(' ').collect_vec();

        let first_token = *tokens.get(0).unwrap();
        let second_token = *tokens.get(1).unwrap();

        // NOT is the only token that comes in the first position
        if first_token == "NOT" {
            let src = tokens.get(1).unwrap().into();
            let dst = tokens.get(3).unwrap().into();

            return Instruction::Not { src, dst };
        }

        // Handle all the other instructions where the label is at the second position
        match second_token {
            "->" => {
                let value = tokens.get(0).unwrap().into();
                let dst = tokens.get(2).unwrap().into();

                Instruction::Load { dst, value }
            }

            "AND" | "OR" | "LSHIFT" | "RSHIFT" => {
                let src = (tokens.get(0).unwrap().into(), tokens.get(2).unwrap().into());
                let dst = tokens.get(4).unwrap().into();

                match second_token {
                    "AND" => Instruction::And { src, dst },
                    "OR" => Instruction::Or { src, dst },
                    "LSHIFT" => Instruction::LShift { src, dst },
                    "RSHIFT" => Instruction::RShift { src, dst },
                    _ => panic!("lol"),
                }
            }

            _ => panic!("lol"),
        }
    }
}

#[derive(Default)]
pub struct CPU {
    state: HashMap<Wire, u16>,
}

impl CPU {
    fn dump_wires(&self) {
        for (wire, value) in &self.state {
            if let Wire::Named(name) = wire {
                println!("{name}: {value}");
            }
        }
    }

    fn exec(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Load { dst, value } => {
                let value = value.get_number(self);
                self.state.insert(dst.clone(), value);
            }

            Instruction::And { src, dst } => {
                let src = (src.0.get_number(self), src.1.get_number(self));
                self.state.insert(dst.clone(), src.0 & src.1);
            }

            Instruction::Or { src, dst } => {
                let src = (src.0.get_number(self), src.1.get_number(self));
                self.state.insert(dst.clone(), src.0 | src.1);
            }

            Instruction::Not { src, dst } => {
                let src = src.get_number(self);
                self.state.insert(dst.clone(), !src);
            }

            Instruction::LShift { src, dst } => {
                let src = (src.0.get_number(self), src.1.get_number(self));
                self.state.insert(dst.clone(), src.0 << src.1);
            }

            Instruction::RShift { src, dst } => {
                let src = (src.0.get_number(self), src.1.get_number(self));
                self.state.insert(dst.clone(), src.0 >> src.1);
            }
        }
    }

    fn get(&self, wire: &Wire) -> u16 {
        self.state
            .get(wire)
            .map(ToOwned::to_owned)
            .unwrap_or_default()
    }
}

pub struct Seven;

impl Solution for Seven {
    type Output = u16;
    type Parsed = Vec<Instruction>;

    fn input() -> &'static str {
        // return "123 -> x
        // 456 -> y
        // x AND y -> d
        // x OR y -> e
        // x LSHIFT 2 -> f
        // y RSHIFT 2 -> g
        // NOT x -> h
        // NOT y -> i";

        include_str!("../inputs/7.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Instruction::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut cpu = CPU::default();

        for instruction in parsed {
            cpu.exec(instruction);
        }

        cpu.dump_wires();

        cpu.get(&Wire::Named("a".to_string()))
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
