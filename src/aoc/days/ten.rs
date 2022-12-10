use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    vec,
};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Ten;

#[derive(Debug, Clone, Copy)]
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

pub struct Matrix {
    inner: Vec<bool>,
    width: usize,
}

impl Matrix {
    fn new(width: usize, height: usize) -> Self {
        Matrix {
            inner: vec![false; width * height],
            width,
        }
    }

    fn set_pixel(&mut self, index: usize, condition: bool) {
        let ciao = self.inner.get_mut(index).unwrap();
        *ciao = condition;
    }

    fn crt_cast(&mut self, reg_x: i32, crt_pos: usize) {
        let sprite_indices: HashSet<i32> = [reg_x - 1, reg_x, reg_x + 1].into();

        if sprite_indices.contains(&(crt_pos as i32 % 40)) {
            self.set_pixel(crt_pos, true);
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars_in_chunks = self
            .inner
            .iter()
            .map(|b| if *b { "#" } else { " " })
            .chunks(self.width);

        let lines = chars_in_chunks.into_iter().map(|s| {
            let mut line = String::from_iter(s);
            line.push('\n');
            line
        });

        write!(f, "{}", String::from_iter(lines))
    }
}

impl Solution for Ten {
    type Output = i32;
    type Parsed = Vec<Instruction>;

    fn input() -> &'static str {
        include_str!("../inputs/10.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Instruction::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let instructions_flat = parsed.into_iter().flat_map(|a| a.into_iter()).collect_vec();

        let mut reg_x = 1;
        let mut values = vec![reg_x];

        for instruction in instructions_flat {
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

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut matrix = Matrix::new(40, 6);
        let mut reg_x: i32 = 1;

        let mut crt_pos = 0; // also a program counter

        for instruction in parsed {
            match instruction {
                Instruction::Addx(increase) => {
                    matrix.crt_cast(reg_x, crt_pos);
                    crt_pos += 1;

                    matrix.crt_cast(reg_x, crt_pos);
                    reg_x += increase;
                }
                Instruction::Noop => {
                    matrix.crt_cast(reg_x, crt_pos);
                }
            }

            crt_pos += 1;
        }

        println!("{}", matrix);

        // expected solution is
        // ###  ###    ## #    #### #  # #    ###
        // #  # #  #    # #    #    #  # #    #  #
        // ###  #  #    # #    ###  #  # #    #  #
        // #  # ###     # #    #    #  # #    ###
        // #  # # #  #  # #    #    #  # #    #
        // ###  #  #  ##  #### #     ##  #### #

        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (12980, 0)
    }
}
