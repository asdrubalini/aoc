#![allow(dead_code)]

use itertools::Itertools;

fn solve<S: AsRef<str>>(input: S) -> usize {
    let input = input.as_ref();

    // Split by line
    let items = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());

    items
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let input = include_str!("./inputs/1.txt");
        assert_eq!(solve(input), 1527);
    }
}
