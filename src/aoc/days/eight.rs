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
            .map(|line| line.chars().collect_vec())
            .flatten()
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

        for y in 1..parsed.height - 1 {
            for x in 1..parsed.width - 1 {
                let current_height = parsed.at(x, y);

                let visible_from_top = (0..y)
                    .map(|sliding_y| parsed.at(x, sliding_y))
                    .filter(|height| *height >= current_height)
                    .next()
                    .is_none();

                if visible_from_top {
                    visible_count += 1;
                    continue;
                }

                let visible_from_bottom = (y + 1..parsed.height)
                    .map(|sliding_y| parsed.at(x, sliding_y))
                    .filter(|height| *height >= current_height)
                    .next()
                    .is_none();

                if visible_from_bottom {
                    visible_count += 1;
                    continue;
                }

                let visible_from_left = (0..x)
                    .map(|sliding_x| parsed.at(sliding_x, y))
                    .filter(|height| *height >= current_height)
                    .next()
                    .is_none();

                if visible_from_left {
                    visible_count += 1;
                    continue;
                }

                let visible_from_right = (x + 1..parsed.width)
                    .map(|sliding_x| parsed.at(sliding_x, y))
                    .filter(|height| *height >= current_height)
                    .next()
                    .is_none();

                if visible_from_right {
                    visible_count += 1;
                    continue;
                }
            }
        }

        visible_count as u32
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1805, 0)
    }
}
