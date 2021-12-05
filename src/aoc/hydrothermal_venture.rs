use std::{
    fmt::{Debug, Display},
    mem::swap,
};

use itertools::Itertools;

use super::Solution;

pub struct DayFive;

#[derive(Debug, PartialEq, PartialOrd)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(item: &str) -> Self {
        let mut coord = item.split(',');

        Self {
            x: coord.next().unwrap().parse().unwrap(),
            y: coord.next().unwrap().parse().unwrap(),
        }
    }
}

impl DayFive {
    fn parse_input(input: &str) -> Vec<(Position, Position)> {
        input
            .lines()
            .map(|line| {
                let mut s = line.split(" -> ");

                let mut begin = Position::new(s.next().unwrap());
                let mut end = Position::new(s.next().unwrap());

                if begin > end {
                    swap(&mut begin, &mut end);
                }

                (begin, end)
            })
            .collect()
    }
}

struct Matrix {
    inner: Vec<Vec<u32>>,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = self
            .inner
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| format!("{} ", item))
                    .collect::<String>()
            })
            .join("\n");

        write!(f, "{}", display_str)
    }
}

impl Matrix {
    fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            inner: vec![vec![0; size_x]; size_y],
        }
    }

    fn increment(&mut self, pos: &Position) {
        self.inner[pos.y as usize][pos.x as usize] += 1;
    }
}

impl Solution for DayFive {
    type Output = u32;

    fn input() -> &'static str {
        include_str!("./inputs/example.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let positions = Self::parse_input(input)
            .into_iter()
            // Only get horizontal or vertical segments
            .filter(|(begin, end)| begin.x == end.x || begin.y == end.y)
            .collect::<Vec<_>>();

        let flattened = positions
            .iter()
            .map(|(begin, end)| vec![begin, end])
            .flatten();

        let max_x = flattened.clone().map(|pos| pos.x).max().unwrap() as usize + 1;
        let max_y = flattened.map(|pos| pos.y).max().unwrap() as usize + 1;

        let mut matrix = Matrix::new(max_x, max_y);

        for (begin, end) in positions.iter() {
            let positions = if begin.x == end.x {
                (begin.y..=end.y)
                    .into_iter()
                    .map(|y| Position { x: begin.x, y })
                    .collect::<Vec<_>>()
            } else if begin.y == end.y {
                (begin.x..=end.x)
                    .into_iter()
                    .map(|x| Position { x, y: begin.y })
                    .collect::<Vec<_>>()
            } else {
                panic!("Should not happen");
            };

            for pos in positions {
                matrix.increment(&pos);
            }
        }

        matrix
            .inner
            .iter()
            .flatten()
            .filter(|item| **item >= 2)
            .count() as u32
    }

    fn solve_second(input: &str) -> Self::Output {
        let positions = Self::parse_input(input);

        let flattened = positions
            .iter()
            .map(|(begin, end)| vec![begin, end])
            .flatten();

        let max_x = flattened.clone().map(|pos| pos.x).max().unwrap() as usize + 1;
        let max_y = flattened.map(|pos| pos.y).max().unwrap() as usize + 1;

        let mut matrix = Matrix::new(max_x, max_y);

        for (begin, end) in positions.iter() {
            let delta_x = end.x - begin.x;
            let delta_y = end.y - begin.y;

            let positions = (0..delta_x)
                .into_iter()
                .map(|delta| Position {
                    x: begin.x + delta,
                    y: begin.y + delta,
                })
                .collect::<Vec<_>>();

            for pos in positions {
                matrix.increment(&pos);
            }
        }

        println!("{}", matrix);

        matrix
            .inner
            .iter()
            .flatten()
            .filter(|item| **item >= 2)
            .count() as u32
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (5835, 0)
    }
}
