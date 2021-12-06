#![allow(dead_code, unused_imports)]

use advent_of_code_2021::aoc::*;

#[cfg(debug_assertions)]
fn main() {}

#[cfg(not(debug_assertions))]
fn main() {
    println!("# Benchmarks:");

    println!("| Day | First part | Second part |");
    println!("| --- | --- | --- |");

    DayOne::benchmark();
    DayTwo::benchmark();
    DayThree::benchmark();
    DayFour::benchmark();
    DayFive::benchmark();
    DaySix::benchmark();
}
