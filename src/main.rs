#![allow(dead_code)]

use advent_of_code_2021::aoc::{DaySix, Solution};

fn main() {
    let input = DaySix::input();

    let first = DaySix::solve_first(input);
    let second = DaySix::solve_second(input);

    dbg!(first);
    dbg!(second);
}
