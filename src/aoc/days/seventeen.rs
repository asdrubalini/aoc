use std::vec;

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Seventeen;

fn can_target_fit_in_containers(target: u32, containers: &[u32]) -> bool {
    containers.iter().sum::<u32>() == target
}

fn next_choice(choices: &mut [bool]) {
    let steps = choices.len() - 1;

    for i in (0..=steps).rev() {
        let choice = choices.get_mut(i).unwrap();
        *choice = !*choice;

        if *choice {
            break;
        }
    }
}

fn dot_product<'a>(choices: &[bool], containers: &[u32]) -> Vec<u32> {
    containers
        .iter()
        .zip(choices.iter())
        .filter_map(
            |(container, selected)| {
                if *selected {
                    Some(*container)
                } else {
                    None
                }
            },
        )
        .collect_vec()
}

impl Solution for Seventeen {
    type Output = u32;
    type Parsed = Vec<u32>;

    fn input() -> &'static str {
        include_str!("../inputs/17.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(|n| n.parse().unwrap()).collect_vec()
    }

    fn solve_first(containers: &Self::Parsed) -> Self::Output {
        // Brute force all the combinations and count how many are valid
        let mut valid_combinations = 0;
        let mut choices = vec![false; containers.len()];
        let steps = 2u32.pow(choices.len() as u32);

        for _ in 0..steps {
            if can_target_fit_in_containers(150, &dot_product(&choices, &containers)) {
                valid_combinations += 1;
            }

            next_choice(&mut choices);
        }

        valid_combinations
    }

    fn solve_second(containers: &Self::Parsed) -> Self::Output {
        // Brute force all the combinations and count how many are valid
        let mut choices = vec![false; containers.len()];
        let steps = 2u32.pow(choices.len() as u32);

        let mut current_min = usize::MAX;
        let mut min_count = 0;

        for _ in 0..steps {
            let current_containers = dot_product(&choices, &containers);

            if can_target_fit_in_containers(150, &current_containers) {
                let containers_len = current_containers.len();

                if containers_len == current_min {
                    min_count += 1;
                } else if containers_len < current_min {
                    current_min = containers_len;
                    min_count = 1;
                }
            }

            next_choice(&mut choices);
        }

        min_count
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1638, 17)
    }
}
