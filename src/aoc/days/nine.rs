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

                (Direction::from(direction), steps.parse().unwrap())
            })
            .collect_vec();

        Motions { motions }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(i32, i32);

impl Coord {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    fn go_to_direction(&mut self, direction: Direction) -> Coord {
        let previous = *self;

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

    fn distance(&self, other: &Self) -> f64 {
        (((self.x() - other.x()).pow(2) + (self.y() - other.y()).pow(2)) as f64).sqrt()
    }

    /// self is touching other when their distance is less or equal than sqrt(2)
    fn is_touching(&self, other: &Self) -> bool {
        let x = self.distance(other) <= f64::sqrt(2.0);

        //println!("is_touching({:?}, {:?}) = {}", self, other, x);

        x
    }
}

impl Solution for Nine {
    type Output = u32;
    type Parsed = Motions;

    fn input() -> &'static str {
        //"R 100\nU 10"
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
                let dx = (head_coords.x() - tail_coords.x()).signum();
                let dy = (head_coords.y() - tail_coords.y()).signum();

                tail_coords.move_by(dx, dy);
                visited_by_tail.push(tail_coords);
            }
        }

        visited_by_tail.into_iter().unique().count() as u32
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut head_coords = Coord(0, 0);
        let mut other_coords = vec![Coord(0, 0); 9];

        let mut visited_by_tail: Vec<Coord> = vec![Coord(0, 0)];

        for (direction, steps) in parsed.motions.iter() {
            for _ in 0..*steps {
                // move head first
                head_coords.go_to_direction(*direction);

                let mut prev_coords = head_coords;

                // then adjust the other ropes
                for (idx, middle_coords) in other_coords.iter_mut().enumerate() {
                    // then move tail in order to follow head
                    while !prev_coords.is_touching(&middle_coords) {
                        let dx = (prev_coords.x() - middle_coords.x()).signum();
                        let dy = (prev_coords.y() - middle_coords.y()).signum();

                        println!("{dx} {dy}");

                        middle_coords.move_by(dx, dy);
                    }

                    if idx == 8 {
                        // 8 is the tail
                        visited_by_tail.push(*middle_coords);
                    }

                    prev_coords = *middle_coords;
                }
            }
        }

        println!("{:?}", head_coords);
        println!("{:?}", other_coords);

        visited_by_tail.into_iter().unique().count() as u32
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (6190, 2516)
    }
}
