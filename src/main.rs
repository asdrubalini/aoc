mod aoc;
use aoc::{solutions::*, Solution};

fn main() {
    let input = DayTwo::input();
    let second = DayTwo::solve_second(&input);

    println!("first: {}", second);
}
