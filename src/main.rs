#![allow(dead_code, unused_imports)]

use advent_of_code_2021::aoc::{days::Two, Solution};

mod aoc;

fn main() {
    let input = Two::input();
    let parsed = Two::parse_input(input);
    let solution = Two::solve_second(&parsed);

    println!("{:?}", solution);
}
