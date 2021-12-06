/// Hydrothermal venture
///
/// We have a list of segments defined by the coordinates of the two extreme points on a 2D matrix.
use std::fmt::{Debug, Display};

use itertools::Itertools;

use super::Solution;

pub struct DayFive;

#[derive(Debug, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(item: &str) -> Self {
        let mut coord = item.split(',');

        Self {
            x: coord.next().unwrap().parse().unwrap(),
            y: coord.next().unwrap().parse().unwrap(),
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

impl DayFive {
    fn parse_input(input: &str) -> Vec<(Position, Position)> {
        input
            .lines()
            .map(|line| {
                let mut s = line.split(" -> ");

                let begin = Position::new(s.next().unwrap());
                let end = Position::new(s.next().unwrap());

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
                    .map(|item| {
                        if *item == 0 {
                            ".".to_string()
                        } else {
                            format!("{}", item)
                        }
                    })
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

    fn get_touch_points(&self, begin: &Position, end: &Position) -> Vec<Position> {
        let total_distance = begin.distance(end);
        let mut touch_points = vec![];

        // Very slow version which checks every point on the matrix
        for (x, row) in (&self.inner).iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                // Here x and y is every point in the matrix
                let current_point = Position {
                    x: x as i32,
                    y: y as i32,
                };

                let difference =
                    current_point.distance(begin) + current_point.distance(end) - total_distance;

                println!("{}", difference);

                if difference > -f64::EPSILON && difference < f64::EPSILON {
                    touch_points.push(current_point);
                }
            }
        }

        touch_points
    }

    fn put_positions(&mut self, positions: &[(Position, Position)]) {
        for (begin, end) in positions {
            // Find which point belonging to a segment touches the matrix
            let touch_points = self.get_touch_points(begin, end);

            for point in touch_points {
                self.increment(&point);
            }
        }
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
        matrix.put_positions(&positions);

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
        matrix.put_positions(&positions);

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
