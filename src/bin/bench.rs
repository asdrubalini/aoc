use advent_of_code_2021::aoc::{days::*, *};

fn main() {
    use std::time::Duration;

    println!("# Advent of Code 2022");
    println!("Run tests: `cargo test`\n");
    println!("Generate benchmarks: `cargo run --bin bench`\n");

    println!("# Benchmarks:");

    println!("| Day | First part | Second part |");
    println!("| --- | --- | --- |");

    let total_time_ns = One::benchmark();
        // + Two::benchmark()
        // + Three::benchmark()
        // + Four::benchmark()
        // + Five::benchmark()
        // + Six::benchmark()
        // + Seven::benchmark()
        // + Eight::benchmark()
        // + Nine::benchmark()
        // + Ten::benchmark()
        // + Eleven::benchmark()
        // + Twelve::benchmark()
        // + Thirteen::benchmark()
        // + Fourteen::benchmark()
        // + Fifteen::benchmark()
        // + Sixteen::benchmark()
        // + Seventeen::benchmark()
        // + Eighteen::benchmark()
        // + Nineteen::benchmark()
        // + Twenty::benchmark()
        // + TwentyOne::benchmark()
        // + TwentyTwo::benchmark()
        // + TwentyThree::benchmark()
        // + TwentyFour::benchmark()
        // + TwentyFive::benchmark();

    let total_time = Duration::from_nanos(total_time_ns);

    println!("\n\nTotal: `{:?}`", total_time);
}
