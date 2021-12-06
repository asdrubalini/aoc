/// Hydrothermal venture
///
/// We have a list of segments defined by the coordinates of the two extreme points on a 2D matrix.
use std::fmt::{Debug, Display};

use itertools::Itertools;

use super::Solution;

pub struct DayFive;

#[derive(Debug, PartialEq, PartialOrd)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn parse_line(item: &str) -> Self {
        let mut coord = item.split(',');

        Self {
            x: coord.next().unwrap().parse().unwrap(),
            y: coord.next().unwrap().parse().unwrap(),
        }
    }

    fn new(x: i32, y: i32) -> Cell {
        Self { x, y }
    }
}

struct Plane {
    inner: Vec<Vec<u32>>,
}

impl Display for Plane {
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

impl Plane {
    fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            inner: vec![vec![0; size_x]; size_y],
        }
    }

    fn increment_cell(&mut self, pos: &Cell) {
        self.inner[pos.y as usize][pos.x as usize] += 1;
    }

    // Get plane points that a segment touches
    fn get_touch_points(&self, segment_begin: &Cell, segment_end: &Cell) -> Vec<Cell> {
        let delta_x = (segment_end.x - segment_begin.x).abs();
        let delta_y = (segment_end.y - segment_begin.y).abs();

        let inc_x = if segment_end.x - segment_begin.x > 0 {
            1
        } else if segment_end.x - segment_begin.x < 0 {
            -1
        } else {
            0
        };

        let inc_y = if segment_end.y - segment_begin.y > 0 {
            1
        } else if segment_end.y - segment_begin.y < 0 {
            -1
        } else {
            0
        };

        if delta_x == 0 && delta_y > 0 {
            // Vertical
            (0..=delta_y)
                .map(|pos_y| Cell::new(segment_begin.x, segment_begin.y + (pos_y * inc_y)))
                .collect()
        } else if delta_y == 0 && delta_x > 0 {
            // Horizontal
            (0..=delta_x)
                .map(|pos_x| Cell::new(segment_begin.x + (pos_x * inc_x), segment_begin.y))
                .collect()
        } else {
            // Diagonal
            todo!()
        }
    }

    fn increment_positions(&mut self, positions: &[(Cell, Cell)]) {
        for (begin, end) in positions {
            // Find which point belonging to the current segment touches the plane
            // and increment each cell accordingly
            for point in self.get_touch_points(begin, end) {
                self.increment_cell(&point);
            }
        }
    }
}

impl DayFive {
    fn parse_input(input: &str) -> Vec<(Cell, Cell)> {
        input
            .lines()
            .map(|line| {
                let mut s = line.split(" -> ");

                let begin = Cell::parse_line(s.next().unwrap());
                let end = Cell::parse_line(s.next().unwrap());

                (begin, end)
            })
            .collect()
    }

    fn plane_size(segments: &[(Cell, Cell)]) -> (usize, usize) {
        let flattened = segments
            .iter()
            .map(|(begin, end)| vec![begin, end])
            .flatten();

        let max_x = flattened.clone().map(|pos| pos.x).max().unwrap() as usize + 1;
        let max_y = flattened.map(|pos| pos.y).max().unwrap() as usize + 1;

        (max_x, max_y)
    }
}

impl Solution for DayFive {
    type Output = u32;

    fn input() -> &'static str {
        include_str!("./inputs/5.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let segments = Self::parse_input(input)
            .into_iter()
            // Only get horizontal or vertical segments, filter out diagonals
            .filter(|(begin, end)| begin.x == end.x || begin.y == end.y)
            .collect::<Vec<_>>();

        let (size_x, size_y) = Self::plane_size(&segments);

        let mut matrix = Plane::new(size_x, size_y);
        matrix.increment_positions(&segments);

        println!("Matrix for first {}", matrix);

        matrix
            .inner
            .iter()
            .flatten()
            .filter(|item| **item >= 2)
            .count() as u32
    }

    fn solve_second(input: &str) -> Self::Output {
        let segments = Self::parse_input(input);

        let (size_x, size_y) = Self::plane_size(&segments);

        let mut matrix = Plane::new(size_x, size_y);
        matrix.increment_positions(&segments);

        println!("Matrix for second {}", matrix);

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
