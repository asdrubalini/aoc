use std::vec;

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Seventeen;

enum DoesFit {
    NotEnough,
    Yes,
    TooMuch,
}

fn can_target_fit_in_containers(target: u32, containers: &[u32]) -> DoesFit {
    let capacity = containers.iter().sum::<u32>();
    // dbg!(capacity);

    if capacity == target {
        DoesFit::Yes
    } else if capacity < target {
        DoesFit::NotEnough
    } else {
        DoesFit::TooMuch
    }
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
        let mut containers = containers.to_owned();
        containers.sort();

        // Brute force all the combinations and count how many are valid
        let mut valid_combinations = 0;
        let mut choices = vec![false; containers.len()];
        let steps = 2u32.pow(choices.len() as u32);

        for _ in 0..steps {
            match can_target_fit_in_containers(150, &dot_product(&choices, &containers)) {
                DoesFit::NotEnough | DoesFit::TooMuch => (),
                DoesFit::Yes => valid_combinations += 1,
            };

            next_choice(&mut choices);
        }

        valid_combinations
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1638, 0)
    }
}
