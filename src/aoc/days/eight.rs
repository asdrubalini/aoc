use itertools::Itertools;

use crate::aoc::Solution;

pub struct Eight;

#[derive(Debug)]
pub struct Matrix {
    width: usize,
    height: usize,
    inner: Vec<i32>,
}

impl From<&str> for Matrix {
    fn from(from: &str) -> Self {
        let inner = from
            .lines()
            .flat_map(|line| line.chars().collect_vec())
            .map(|c| c.to_digit(10).expect("that's not a number lol") as i32)
            .collect_vec();

        let width = from.lines().next().unwrap().chars().count();
        let height = from.lines().count();

        assert_eq!(inner.len(), width * height);

        Matrix {
            width,
            height,
            inner,
        }
    }
}

impl Matrix {
    fn at(&self, x: usize, y: usize) -> i32 {
        match self.inner.get(y * self.width + x) {
            Some(value) => *value,
            None => panic!("cannot find"),
        }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let current_height = self.at(x, y);

        let obstructed_from_top = (0..y)
            .map(|sliding_y| self.at(x, sliding_y))
            .any(|height| height >= current_height);

        if !obstructed_from_top {
            return true;
        }

        let obstructed_from_bottom = (y + 1..self.height)
            .map(|sliding_y| self.at(x, sliding_y))
            .any(|height| height >= current_height);

        if !obstructed_from_bottom {
            return true;
        }

        let obstructed_from_left = (0..x)
            .map(|sliding_x| self.at(sliding_x, y))
            .any(|height| height >= current_height);

        if !obstructed_from_left {
            return true;
        }

        let obstructed_from_right = (x + 1..self.width)
            .map(|sliding_x| self.at(sliding_x, y))
            .any(|height| height >= current_height);

        if !obstructed_from_right {
            return true;
        }

        false
    }
}

impl Solution for Eight {
    type Output = u32;
    type Parsed = Matrix;

    fn input() -> &'static str {
        include_str!("../inputs/8.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        Matrix::from(input)
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        // count the edges
        let mut visible_count = parsed.height * 2 + (parsed.width - 2) * 2;

        // iterate over every single tree (except the edges) and count how many of those
        // are visible from outside
        visible_count += (1..parsed.height - 1)
            .flat_map(|y| (1..parsed.width - 1).map(move |x| parsed.is_visible(x, y)))
            .filter(|is_visible| *is_visible)
            .count();

        visible_count as u32
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1805, 0)
    }
}
