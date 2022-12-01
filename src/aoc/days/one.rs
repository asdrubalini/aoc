use itertools::Itertools;

use crate::aoc::Solution;

pub struct One;

impl Solution for One {
    type Output = u32;
    type Parsed = Vec<Vec<u32>>;

    fn input() -> &'static str {
        include_str!("../inputs/1.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        let input_lines = input.lines();
        let mut elves: Vec<Vec<u32>> = vec![vec![]];

        for line in input_lines {
            if line.is_empty() {
                elves.push(vec![]);
                continue;
            }

            elves.last_mut().unwrap().push(line.parse().unwrap());
        }

        elves
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let ordered_sums: Vec<u32> = parsed
            .iter()
            .map(|s| s.iter().sum::<u32>())
            .sorted()
            .rev()
            .collect();

        *ordered_sums.first().unwrap()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let ordered_sums: Vec<u32> = parsed
            .iter()
            .map(|s| s.iter().sum::<u32>())
            .sorted()
            .rev()
            .collect();

        ordered_sums.iter().take(3).sum::<u32>()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (74711, 209481)
    }
}
