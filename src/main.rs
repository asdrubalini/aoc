mod aoc;
use aoc::{solutions::*, Solution};

fn main() {
    let input = DayTwo::input();
    let first = DayTwo::solve_first(&input);

    println!("first: {}", first);
}
