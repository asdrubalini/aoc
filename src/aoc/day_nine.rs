use super::Solution;

pub struct DayNine;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn to_right(mut self) -> Self {
        self.x += 1;
        self
    }

    fn to_left(mut self) -> Self {
        self.x -= 1;
        self
    }

    fn to_top(mut self) -> Self {
        self.y += 1;
        self
    }

    fn to_bottom(mut self) -> Self {
        self.y -= 1;
        self
    }
}

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
    fn get(&self, point: Point) -> u8 {
        self.inner[point.y][point.x]
    }

    fn get_neighbor(&self, point: Point) -> Vec<u8> {
        let mut neighbors = vec![];

        if point.x > 0 {
            neighbors.push(self.get(point.to_left()));
        }

        if point.x + 1 < self.width {
            neighbors.push(self.get(point.to_right()));
        }

        if point.y > 0 {
            neighbors.push(self.get(point.to_bottom()));
        }

        if point.y + 1 < self.height {
            neighbors.push(self.get(point.to_top()));
        }

        neighbors
    }

    fn neighbors(&self) -> Neighbors {
        Neighbors {
            matrix: self,
            current_point: Point { x: 0, y: 0 },
        }
    }
}

struct Neighbors<'a> {
    matrix: &'a Matrix,
    current_point: Point,
}

impl Iterator for Neighbors<'_> {
    type Item = ((u8, Point), Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point.x == self.matrix.width {
            self.current_point.x = 0;
            self.current_point.y += 1;
        }

        if self.current_point.y == self.matrix.height {
            return None;
        }

        let current_cell = (self.matrix.get(self.current_point), self.current_point);
        let neighbors = self.matrix.get_neighbor(self.current_point);

        self.current_point.x += 1;
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

        // Find all the minimum points and compute their sum
        matrix
            .neighbors()
            .into_iter()
            .filter(|(point, neighbors)| neighbors.iter().all(|neighbor| point.0 < *neighbor))
            .map(|(point, _)| (point.0 as u32) + 1)
            .sum()
    }

    fn solve_second(input: &str) -> Self::Output {
        let matrix = Matrix::new(input);

        // Find all the minimum points and their coordinates
        let min_points = matrix
            .neighbors()
            .into_iter()
            .filter(|(point, neighbors)| neighbors.iter().all(|neighbor| point.0 < *neighbor))
            .map(|(point, _)| (point.1))
            .collect::<Vec<_>>();

        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (516, 0)
    }
}
