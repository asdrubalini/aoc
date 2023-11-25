#![allow(dead_code)]

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Twenty;

#[derive(Default)]
pub struct World {
    elves: Vec<()>,
    houses: Vec<u32>,
}

impl World {
    fn new() -> Self {
        todo!();
    }
}

impl World {
    fn tick(&mut self) {
        todo!()
    }
}

impl Solution for Twenty {
    type Output = u32;
    type Parsed = u32;

    fn input() -> &'static str {
        include_str!("../inputs/20.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.parse().unwrap()
    }

    fn solve_first(target: &Self::Parsed) -> Self::Output {
        let mut world = World::default();

        loop {
            world.tick();

            let (i, presents_count) = world.houses.iter().sorted().enumerate().next().unwrap();

            if presents_count >= target {
                return i as u32 + 1;
            }
        }
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
