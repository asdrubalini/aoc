#![allow(dead_code, unused_imports)]

use advent_of_code_2021::aoc::{days::One, Solution};

mod aoc;

fn main() {
    let input = One::input();
    let solution = One::solve_first(input);

    println!("{}", solution);
}
