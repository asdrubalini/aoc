use std::{
    any::type_name,
    fmt::Debug,
    time::{Duration, Instant},
};

mod sonar_sweep;
pub use sonar_sweep::DayOne;

mod dive;
pub use dive::DayTwo;

mod binary_diagnostic;
pub use binary_diagnostic::DayThree;

mod bingo;
pub use bingo::DayFour;

mod hydrothermal_venture;
pub use hydrothermal_venture::DayFive;

mod lanternfish;
pub use lanternfish::DaySix;

const SAMPLE_SIZE: usize = 128;

pub trait Solution {
    type Output: Eq + Debug;

    fn input() -> &'static str;
    fn solve_first(input: &str) -> Self::Output;
    fn solve_second(input: &str) -> Self::Output;
    fn expected_solutions() -> (Self::Output, Self::Output);

    fn assert_solutions() {
        let input = Self::input();

        let first = Self::solve_first(input);
        let second = Self::solve_second(input);

        assert_eq!(first, Self::expected_solutions().0);
        assert_eq!(second, Self::expected_solutions().1);
    }

    fn benchmark() {
        let type_name = type_name::<Self>().split("::").last().unwrap();

        println!("## Benchmarking {}...", type_name);
        print!("```");

        let input = Self::input();

        let elapsed: f64 = (0..SAMPLE_SIZE)
            .into_iter()
            .map(|_| {
                let begin = Instant::now();
                let _ = Self::solve_first(input);
                begin.elapsed().as_nanos()
            })
            .sum::<u128>() as f64
            / SAMPLE_SIZE as f64;
        let elapsed = Duration::from_nanos(elapsed.round() as u64);

        println!("part_one...\t{:?} to run", elapsed);

        let elapsed: f64 = (0..SAMPLE_SIZE)
            .into_iter()
            .map(|_| {
                let begin = Instant::now();
                let _ = Self::solve_second(input);
                begin.elapsed().as_nanos()
            })
            .sum::<u128>() as f64
            / SAMPLE_SIZE as f64;
        let elapsed = Duration::from_nanos(elapsed.round() as u64);

        println!("part_two...\t{:?} to run", elapsed);
        println!("```");
    }
}
