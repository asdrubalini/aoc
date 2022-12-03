use std::{
    any::type_name,
    fmt::Debug,
    time::{Duration, Instant},
};

pub mod days;

const SAMPLE_SIZE: usize = 1024;

pub trait Solution {
    type Output: Eq + Debug;
    type Parsed: Debug;

    /// Provide input for the problem
    fn input() -> &'static str;

    /// Parse input and provide a custom version of type Self::Parsed
    fn parse_input(_input: &'static str) -> Self::Parsed;

    /// Solve first problem an provide an output
    fn solve_first(input: &Self::Parsed) -> Self::Output;

    /// Solve second problem and provide an output
    fn solve_second(input: &Self::Parsed) -> Self::Output;

    /// Provide expected solutions for the day in order to make sure that tests are ok
    fn expected_solutions() -> (Self::Output, Self::Output);

    /// panic! if solutions are wrong
    fn assert_solutions() {
        let input = Self::input();
        let parsed = Self::parse_input(input);

        let first = Self::solve_first(&parsed);
        let second = Self::solve_second(&parsed);

        assert_eq!(first, Self::expected_solutions().0);
        assert_eq!(second, Self::expected_solutions().1);
    }

    /// Measure how many nanoseconds solutions take to run
    fn benchmark() -> u64 {
        let type_name = type_name::<Self>().split("::").last().unwrap();

        print!("| {} ", type_name);

        let input = Self::input();
        let parsed = Self::parse_input(input);

        let elapsed: f64 = (0..SAMPLE_SIZE)
            .into_iter()
            .map(|_| {
                let begin = Instant::now();
                let _ = Self::solve_first(&parsed);
                begin.elapsed().as_nanos()
            })
            .sum::<u128>() as f64
            / SAMPLE_SIZE as f64;
        let elapsed_first = Duration::from_nanos(elapsed.round() as u64);

        print!("| {:?} ", elapsed_first);

        let elapsed: f64 = (0..SAMPLE_SIZE)
            .into_iter()
            .map(|_| {
                let begin = Instant::now();
                let _ = Self::solve_second(&parsed);
                begin.elapsed().as_nanos()
            })
            .sum::<u128>() as f64
            / SAMPLE_SIZE as f64;
        let elapsed_second = Duration::from_nanos(elapsed.round() as u64);

        println!("| {:?} |", elapsed_second);

        (elapsed_first.as_nanos() + elapsed_second.as_nanos()) as u64
    }
}
