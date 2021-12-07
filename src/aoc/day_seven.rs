use super::Solution;

pub struct DaySeven;

impl DaySeven {
    fn parse_input(input: &str) -> Vec<u64> {
        input.split(',').map(|item| item.parse().unwrap()).collect()
    }

    fn compute_cost(position: u64, initial_positions: &[u64]) -> u64 {
        initial_positions
            .into_iter()
            .map(|initial_pos| (*initial_pos as i32 - position as i32).abs() as u64)
            .sum()
    }
}

impl Solution for DaySeven {
    type Output = u64;

    fn input() -> &'static str {
        include_str!("./inputs/7.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let input = Self::parse_input(input);

        let max_width = *input.iter().max().unwrap();

        (0..=max_width)
            .into_iter()
            .map(|pos| Self::compute_cost(pos, &input))
            .min()
            .unwrap()
    }

    fn solve_second(input: &str) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (348664, 0)
    }
}
