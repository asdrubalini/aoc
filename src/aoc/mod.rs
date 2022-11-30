use std::{
    any::type_name,
    fmt::Debug,
    time::{Duration, Instant},
};

pub mod days;

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

    fn benchmark() -> u64 {
        let type_name = type_name::<Self>().split("::").last().unwrap();

        print!("| {} ", type_name);

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
        let elapsed_first = Duration::from_nanos(elapsed.round() as u64);

        print!("| {:?} ", elapsed_first);

        let elapsed: f64 = (0..SAMPLE_SIZE)
            .into_iter()
            .map(|_| {
                let begin = Instant::now();
                let _ = Self::solve_second(input);
                begin.elapsed().as_nanos()
            })
            .sum::<u128>() as f64
            / SAMPLE_SIZE as f64;
        let elapsed_second = Duration::from_nanos(elapsed.round() as u64);

        println!("| {:?} |", elapsed_second);

        (elapsed_first.as_nanos() + elapsed_second.as_nanos()) as u64
    }
}
