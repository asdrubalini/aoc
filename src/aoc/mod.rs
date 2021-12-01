use std::fmt::Debug;

pub mod solutions;

pub trait Solution {
    type Output: Eq + Debug;

    fn input() -> String;
    fn solve<S: AsRef<str>>(input: S) -> Self::Output;
    fn expected_solutions() -> Self::Output;

    fn assert_solutions() {
        let input = Self::input();
        let solution = Self::solve(input);
        assert_eq!(solution, Self::expected_solutions());
    }
}
