use core::panic;

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Nine;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(from: &str) -> Self {
        match from {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("not a direction"),
        }
    }
}

#[derive(Debug)]
pub struct Motions {
    motions: Vec<(Direction, u32)>,
}

impl From<&str> for Motions {
    fn from(from: &str) -> Self {
        let motions = from
            .lines()
            .map(|line| {
                let (direction, steps) = line.split_ascii_whitespace().collect_tuple().unwrap();

                (
                    Direction::from(direction),
                    u32::from_str_radix(steps, 10).unwrap(),
                )
            })
            .collect_vec();

        Motions { motions }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    fn go_to_direction(&mut self, direction: Direction) -> Coord {
        let previous = self.clone();

        match direction {
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
        }

        previous
    }

    fn move_by(&mut self, x: i32, y: i32) {
        self.0 += x;
        self.1 += y;
    }

    pub fn distance(&self, other: &Self) -> f64 {
        (((self.x() - other.x()).pow(2) + (self.y() - other.y()).pow(2)) as f64).sqrt()
    }

    /// self is touching other when their distance is less or equal than sqrt(2)
    pub fn is_touching(&self, other: &Self) -> bool {
        self.distance(other) <= f64::sqrt(2.0)
    }
}

impl Solution for Nine {
    type Output = u32;
    type Parsed = Motions;

    fn input() -> &'static str {
        include_str!("../inputs/9.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        Motions::from(input)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut head_coords = Coord(0, 0);
        let mut tail_coords = Coord(0, 0);

        let mut visited_by_tail: Vec<Coord> = vec![tail_coords];

        for (direction, steps) in parsed.motions.iter() {
            // move head first
            for _ in 0..*steps {
                head_coords.go_to_direction(*direction);
            }

            // then move tail in order to follow head
            while !head_coords.is_touching(&tail_coords) {
                if (head_coords.x() - tail_coords.x()).abs() == 1 {
                    // adjust horizontal position (move diagonally)
                    let movement_x = if head_coords.x() - tail_coords.x() > 0 {
                        1
                    } else {
                        -1
                    };

                    let movement_y = if head_coords.y() - tail_coords.y() > 0 {
                        1
                    } else {
                        -1
                    };

                    tail_coords.move_by(movement_x, movement_y);
                } else if (head_coords.y() - tail_coords.y()).abs() == 1 {
                    // adjust vertical position (move diagonally)
                    let movement_y = if head_coords.y() - tail_coords.y() > 0 {
                        1
                    } else {
                        -1
                    };

                    let movement_x = if head_coords.x() - tail_coords.x() > 0 {
                        1
                    } else {
                        -1
                    };

                    tail_coords.move_by(movement_x, movement_y);
                } else if head_coords.x() != tail_coords.x() && head_coords.y() == tail_coords.y() {
                    // move horizontally
                    let movement_x = if head_coords.x() - tail_coords.x() > 0 {
                        1
                    } else {
                        -1
                    };

                    tail_coords.move_by(movement_x, 0);
                } else if head_coords.y() != tail_coords.y() && head_coords.x() == tail_coords.x() {
                    // move vertically
                    let movement_y = if head_coords.y() - tail_coords.y() > 0 {
                        1
                    } else {
                        -1
                    };

                    tail_coords.move_by(0, movement_y);
                } else {
                    panic!("wtf is this");
                }

                visited_by_tail.push(tail_coords);
            }
        }

        visited_by_tail.into_iter().unique().count() as u32
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
