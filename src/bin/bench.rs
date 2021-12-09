use advent_of_code_2021::aoc::*;

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
        + DaySix::benchmark()
        + DaySeven::benchmark()
        + DayEight::benchmark();

    let total_time = Duration::from_nanos(total_time_ns);

    println!("\n\nTotal: `{:?}`", total_time);
}
