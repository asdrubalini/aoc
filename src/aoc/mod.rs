use std::fmt::Debug;

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
}
