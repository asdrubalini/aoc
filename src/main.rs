#![allow(dead_code)]

mod aoc;
use aoc::{solutions::*, Solution};

fn main() {
    let input = DayFour::input();
    dbg!(DayFour::solve_first(&input));
}
