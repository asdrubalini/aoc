use std::collections::HashMap;

use itertools::Itertools;

use crate::aoc::{
    utils::{Coord, InfiniteMatrix},
    Solution,
};

pub struct Three;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cell {
    UnknownSymbol,
    Gear,
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

                if chr.is_ascii_digit() {
                    current_number.push(*chr);
                } else {
                    if *chr == '.' {
                        transformed_matrix.set(coord, Cell::Dot);
                    } else {
                        transformed_matrix.set(coord, Cell::UnknownSymbol);
                    }

                    if !current_number.is_empty() {
                        let number = current_number.parse::<u32>().unwrap();

                        for i in 1..=current_number.len() {
                            let coord = Coord(coord.x() - i as i32, coord.y());

                            transformed_matrix.set(coord, Cell::PartNumber { number, unique_id });
                        }

                        unique_id += 1;
                        current_number = String::new();
                    }
                }

                last_coord = coord;
            }

            // Handle the edge case when there is a number at the end of the row
            if !current_number.is_empty() {
                let number = current_number.parse::<u32>().unwrap();

                for i in 0..current_number.len() {
                    let coord = Coord(last_coord.x() - i as i32, last_coord.y());

                    transformed_matrix.set(coord, Cell::PartNumber { number, unique_id });
                }

                unique_id += 1;
            }
        }

        let coords_with_unknown_symbols = transformed_matrix
            .coords_iterator()
            .filter(|c| matches!(transformed_matrix.at(*c).unwrap(), Cell::UnknownSymbol));

        let mut neighbors_with_partnumber_unique = HashMap::new();

        for coord in coords_with_unknown_symbols {
            let neighbors = coord.all_neighbors_with_diagonal();
            let neighbors_with_partnumber = neighbors.into_iter().filter_map(|c| {
                if let Some(Cell::PartNumber { number, unique_id }) = transformed_matrix.at(c) {
                    Some((*number, *unique_id))
                } else {
                    None
                }
            });

            for (part_number, unique_id) in neighbors_with_partnumber {
                neighbors_with_partnumber_unique.insert(unique_id, part_number);
            }
        }

        neighbors_with_partnumber_unique.values().sum()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut transformed_matrix = InfiniteMatrix::<Cell>::from_other_matrix_config(parsed);
        let mut unique_id = 0;

        for row in parsed.rows_iterator() {
            let mut current_number = String::new();
            let mut last_coord = *row.first().unwrap();

            for coord in row {
                let chr = parsed.at(coord).unwrap();

                if chr.is_ascii_digit() {
                    current_number.push(*chr);
                } else {
                    if *chr == '.' {
                        transformed_matrix.set(coord, Cell::Dot);
                    } else if *chr == '*' {
                        transformed_matrix.set(coord, Cell::Gear);
                    } else {
                        transformed_matrix.set(coord, Cell::UnknownSymbol);
                    }

                    if !current_number.is_empty() {
                        let number = current_number.parse::<u32>().unwrap();

                        for i in 1..=current_number.len() {
                            let coord = Coord(coord.x() - i as i32, coord.y());

                            transformed_matrix.set(coord, Cell::PartNumber { number, unique_id });
                        }

                        unique_id += 1;
                        current_number = String::new();
                    }
                }

                last_coord = coord;
            }

            // Handle the edge case when there is a number at the end of the row
            if !current_number.is_empty() {
                let number = current_number.parse::<u32>().unwrap();

                for i in 0..current_number.len() {
                    let coord = Coord(last_coord.x() - i as i32, last_coord.y());

                    transformed_matrix.set(coord, Cell::PartNumber { number, unique_id });
                }

                unique_id += 1;
            }
        }

        let coords_with_gears = transformed_matrix
            .coords_iterator()
            .filter(|c| matches!(transformed_matrix.at(*c).unwrap(), Cell::Gear));

        coords_with_gears
            .into_iter()
            .filter_map(|coord| {
                let neighbors = coord.all_neighbors_with_diagonal();
                let neighbors_with_partnumber = neighbors
                    .into_iter()
                    .filter_map(|c| {
                        if let Some(Cell::PartNumber { number, unique_id }) =
                            transformed_matrix.at(c)
                        {
                            Some((*number, *unique_id))
                        } else {
                            None
                        }
                    })
                    .unique()
                    .map(|c| c.0)
                    .collect_vec();

                if neighbors_with_partnumber.len() == 2 {
                    Some(neighbors_with_partnumber[0] * neighbors_with_partnumber[1])
                } else {
                    None
                }
            })
            .sum()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (540212, 87605697)
    }
}
