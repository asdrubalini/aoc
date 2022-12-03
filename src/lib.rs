#![allow(dead_code, unused_macros, unused_variables)]

extern crate core;

pub mod aoc;

/// Generate tests for day n
macro_rules! test_day {
    ($struct_name:ident) => {
        paste! {
            #[test]
            fn [<test_ $struct_name:snake>]() {
                $struct_name::assert_solutions();
            }
        }
    };
}

pub fn run_benchmarks() {
    use aoc::days::*;
    use aoc::Solution;
    use std::time::Duration;

    println!("# Advent of Code 2022");
    println!("Run tests: `cargo test`\n");
    println!("Generate benchmarks: `cargo run --bin bench`\n");

    println!("# Benchmarks:");

    println!("| Day | First part | Second part |");
    println!("| --- | --- | --- |");

    let total_time_ns = One::benchmark()
        + Two::benchmark()
        + Three::benchmark()
        + Four::benchmark()
        + Five::benchmark()
        + Six::benchmark()
        + Seven::benchmark()
        + Eight::benchmark()
        + Nine::benchmark()
        + Ten::benchmark()
        + Eleven::benchmark()
        + Twelve::benchmark()
        + Thirteen::benchmark()
        + Fourteen::benchmark()
        + Fifteen::benchmark()
        + Sixteen::benchmark()
        + Seventeen::benchmark()
        + Eighteen::benchmark()
        + Nineteen::benchmark()
        + Twenty::benchmark()
        + TwentyOne::benchmark()
        + TwentyTwo::benchmark()
        + TwentyThree::benchmark()
        + TwentyFour::benchmark()
        + TwentyFive::benchmark();

    let total_time = Duration::from_nanos(total_time_ns);

    println!("\n\nTotal: `{:?}`", total_time);
}

#[cfg(test)]
mod tests {
    use super::aoc::days::*;
    use super::aoc::Solution;
    use paste::paste;

    test_day!(One);
    test_day!(Two);
    test_day!(Three);
    test_day!(Four);
    test_day!(Five);
    test_day!(Six);
    test_day!(Seven);
    test_day!(Eight);
    test_day!(Nine);
    test_day!(Ten);
    test_day!(Eleven);
    test_day!(Twelve);
    test_day!(Thirteen);
    test_day!(Fourteen);
    test_day!(Fifteen);
    test_day!(Sixteen);
    test_day!(Seventeen);
    test_day!(Eighteen);
    test_day!(Nineteen);
    test_day!(Twenty);
    test_day!(TwentyOne);
    test_day!(TwentyTwo);
    test_day!(TwentyThree);
    test_day!(TwentyFour);
    test_day!(TwentyFive);
}
