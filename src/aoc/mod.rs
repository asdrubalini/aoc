use std::fmt::Debug;

mod sonar_sweep;
pub use sonar_sweep::DayOne;

mod dive;
pub use dive::DayTwo;

mod binary_diagnostic;
pub use binary_diagnostic::DayThree;

mod bingo;
pub use bingo::DayFour;

pub trait Solution {
    type Output: Eq + Debug;

    fn input() -> String;
    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output;
    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output;
    fn expected_solutions() -> (Self::Output, Self::Output);

    fn assert_solutions() {
        let input = Self::input();

        let first = Self::solve_first(&input);
        let second = Self::solve_second(&input);

        assert_eq!(first, Self::expected_solutions().0);
        assert_eq!(second, Self::expected_solutions().1);
    }
}
