#![allow(dead_code, unused_imports)]

use advent_of_code_2021::aoc::*;

#[cfg(debug_assertions)]
fn main() {}

#[cfg(not(debug_assertions))]
fn main() {
    use std::time::Duration;

    println!("# Benchmarks:");

    println!("| Day | First part | Second part |");
    println!("| --- | --- | --- |");

    let total_time_ns = DayOne::benchmark()
        + DayTwo::benchmark()
        + DayThree::benchmark()
        + DayFour::benchmark()
        + DayFive::benchmark()
        + DaySix::benchmark();

    let total_time = Duration::from_nanos(total_time_ns);

    println!("Total: `{:?}`", total_time);
}
