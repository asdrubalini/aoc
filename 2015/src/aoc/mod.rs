use std::fmt::Debug;

pub mod days;
pub mod utils;

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
}
