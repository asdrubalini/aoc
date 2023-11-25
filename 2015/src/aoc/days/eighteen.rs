use std::mem;

use itertools::Itertools;

use crate::aoc::{
    utils::{Coord, InfiniteMatrix},
    Solution,
};

pub struct Eighteen;

impl Solution for Eighteen {
    type Output = u32;
    type Parsed = InfiniteMatrix<bool>;

    fn input() -> &'static str {
        include_str!("../inputs/18.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        let lines = input.lines().collect_vec();
        let width = lines.first().unwrap().len();
        let height = lines.len();

        let mut m = InfiniteMatrix::<bool>::new_fixed(width, height);

        // Start from the bottom
        let lines = lines.into_iter().rev();

        for (y, line) in lines.into_iter().enumerate() {
            for (x, state) in line.chars().enumerate() {
                let state_bool = match state {
                    '#' => true,
                    '.' => false,
                    _ => panic!("lol"),
                };

                m.set(Coord(x as i32, y as i32), state_bool);
            }
        }

        m
    }

    fn solve_first(matrix: &Self::Parsed) -> Self::Output {
        let mut matrix = matrix.to_owned();

        let steps = 100;

        for _ in 0..steps {
            let mut new_matrix = matrix.clone();

            for coord in matrix.coords_iterator() {
                let current_status = *matrix.at(coord).unwrap();

                let neighbors = coord
                    .all_neighbors_with_diagonal()
                    .into_iter()
                    .filter_map(|coord| matrix.at(coord))
                    .copied()
                    .collect_vec();

                let on_neighbors_count = neighbors.into_iter().filter(|state| *state).count();

                *new_matrix.at_mut(coord).unwrap() =
                    on_neighbors_count == 3 || current_status && on_neighbors_count == 2;
            }

            let _ = mem::replace(&mut matrix, new_matrix);
        }

        matrix.into_iter().filter(|(_coord, state)| *state).count() as u32
    }

    fn solve_second(matrix: &Self::Parsed) -> Self::Output {
        let mut matrix = matrix.to_owned();

        let lights_stuck_on = [Coord(0, 0), Coord(99, 0), Coord(99, 99), Coord(0, 99)];

        for coord in lights_stuck_on.iter() {
            matrix.set(*coord, true);
        }

        let steps = 100;

        for _ in 0..steps {
            let mut new_matrix = matrix.clone();

            for coord in matrix.coords_iterator() {
                if lights_stuck_on.contains(&coord) {
                    continue;
                }

                let current_status = *matrix.at(coord).unwrap();

                let neighbors = coord
                    .all_neighbors_with_diagonal()
                    .into_iter()
                    .filter_map(|coord| matrix.at(coord))
                    .copied()
                    .collect_vec();

                let on_neighbors_count = neighbors.into_iter().filter(|state| *state).count();

                *new_matrix.at_mut(coord).unwrap() =
                    on_neighbors_count == 3 || current_status && on_neighbors_count == 2;
            }

            let _ = mem::replace(&mut matrix, new_matrix);
        }

        matrix.into_iter().filter(|(_coord, state)| *state).count() as u32
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (814, 924)
    }
}
