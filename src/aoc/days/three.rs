use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::aoc::{
    utils::{Coord, InfiniteMatrix},
    Solution,
};

pub struct Three;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cell {
    UnknownSymbol,
    Dot,
    PartNumber { number: u32, unique_id: u32 },
}

impl Solution for Three {
    type Output = u32;
    type Parsed = InfiniteMatrix<char>;

    fn input() -> &'static str {
        include_str!("../inputs/3.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        let lines = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        InfiniteMatrix::from(lines)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut transformed_matrix = InfiniteMatrix::<Cell>::from_other_matrix_config(parsed);
        let mut unique_id = 0;

        for row in parsed.rows_iterator() {
            let mut current_number = String::new();
            let mut last_coord = *row.first().unwrap();

            for coord in row {
                let chr = parsed.at(coord).unwrap();

                if chr.is_digit(10) {
                    current_number.push(*chr);
                } else {
                    if *chr == '.' {
                        transformed_matrix.set(coord, Cell::Dot);
                    } else {
                        transformed_matrix.set(coord, Cell::UnknownSymbol);
                    }

                    if current_number.len() > 0 {
                        let n = current_number.parse::<u32>().unwrap();

                        for i in 1..=current_number.len() {
                            let c = Coord(coord.x() - i as i32, coord.y());

                            transformed_matrix.set(
                                c,
                                Cell::PartNumber {
                                    number: n,
                                    unique_id,
                                },
                            );
                        }

                        unique_id += 1;
                        current_number = String::new();
                    }
                }

                last_coord = coord;
            }

            if current_number.len() > 0 {
                let n = current_number.parse::<u32>().unwrap();

                for i in 0..current_number.len() {
                    let c = Coord(last_coord.x() - i as i32, last_coord.y());

                    transformed_matrix.set(
                        c,
                        Cell::PartNumber {
                            number: n,
                            unique_id,
                        },
                    );
                }

                unique_id += 1;
                current_number = String::new();
            }
        }

        let coords_with_unknown_symbols = transformed_matrix
            .coords_iterator()
            .into_iter()
            .filter(|c| matches!(transformed_matrix.at(*c).unwrap(), Cell::UnknownSymbol));

        let mut neighbors_with_partnumber_unique = HashSet::new();

        for coord in coords_with_unknown_symbols {
            let neighbors = coord.all_neighbors_with_diagonal();
            let neighbors_with_partnumber =
                neighbors
                    .into_iter()
                    .filter_map(|c| match transformed_matrix.at(c) {
                        Some(cell) => match cell {
                            Cell::UnknownSymbol => None,
                            Cell::Dot => None,
                            Cell::PartNumber { number, unique_id } => Some((*number, *unique_id)),
                        },

                        None => None,
                    });

            for n in neighbors_with_partnumber {
                neighbors_with_partnumber_unique.insert(n);
            }
        }

        neighbors_with_partnumber_unique
            .into_iter()
            .map(|c| c.0) // sum part numbers
            .sum()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (540212, 0)
    }
}
