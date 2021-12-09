use super::Solution;

pub struct DayNine;

#[derive(Debug)]
struct Matrix {
    inner: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new(input: &str) -> Self {
        let inner: Vec<Vec<u8>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|height| height.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let width = inner[0].len();
        let height = inner.len();

        Self {
            inner,
            width,
            height,
        }
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> u8 {
        self.inner[y][x]
    }

    fn neighbors(&self) -> Neighbors {
        Neighbors {
            matrix: self,
            current_x: 0,
            current_y: 0,
        }
    }
}

struct Neighbors<'a> {
    matrix: &'a Matrix,
    current_x: usize,
    current_y: usize,
}

impl Iterator for Neighbors<'_> {
    type Item = (u8, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x == self.matrix.width {
            self.current_x = 0;
            self.current_y += 1;
        }

        if self.current_y == self.matrix.height {
            return None;
        }

        let current_cell = self.matrix.get(self.current_x, self.current_y);

        let mut neighbors = vec![];

        if self.current_x > 0 {
            neighbors.push(self.matrix.get(self.current_x - 1, self.current_y));
        }

        if self.current_x + 1 < self.matrix.width {
            neighbors.push(self.matrix.get(self.current_x + 1, self.current_y));
        }

        if self.current_y > 0 {
            neighbors.push(self.matrix.get(self.current_x, self.current_y - 1));
        }

        if self.current_y + 1 < self.matrix.height {
            neighbors.push(self.matrix.get(self.current_x, self.current_y + 1));
        }

        self.current_x += 1;
        Some((current_cell, neighbors))
    }
}

impl Solution for DayNine {
    type Output = u32;

    fn input() -> &'static str {
        include_str!("./inputs/9.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let matrix = Matrix::new(input);

        matrix
            .neighbors()
            .into_iter()
            .filter(|(point, neighbors)| neighbors.into_iter().all(|neighbor| point < neighbor))
            .map(|(point, _)| (point as u32) + 1)
            .sum()
    }

    fn solve_second(input: &str) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (516, 0)
    }
}
