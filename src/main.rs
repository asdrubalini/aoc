#![allow(dead_code)]

mod aoc;
use aoc::{solutions::*, Solution};

fn main() {
    let input = DayThree::input();
    let out = DayThree::solve_first(&input);

    println!("first: {}", out);
}
