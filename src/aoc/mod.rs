pub mod solutions;

pub trait Solution {
    type Output;

    fn input() -> String;
    fn solve<S: AsRef<str>>(input: S) -> Self::Output;
    fn expected_solution() -> Self::Output;
}
