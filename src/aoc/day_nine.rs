use itertools::Itertools;

use super::Solution;

pub struct DayNine;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    height: u8,
}

impl Point {
    #[inline]
    fn to_right(mut self) -> Self {
        self.x += 1;
        self
    }

    #[inline]
    fn to_left(mut self) -> Self {
        self.x -= 1;
        self
    }

    #[inline]
    fn to_top(mut self) -> Self {
        self.y += 1;
        self
    }

    #[inline]
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
    fn point_at(&self, x: usize, y: usize) -> Point {
        Point {
            height: self.inner[y][x],
            x,
            y,
        }
    }

    /// Get `point` neighbors
    #[inline]
    fn get_neighbor(&self, point: Point) -> Vec<Point> {
        let mut neighbors = vec![];

        if point.x > 0 {
            let next_point = point.to_left();
            neighbors.push(self.point_at(next_point.x, next_point.y));
        }

        if point.x + 1 < self.width {
            let next_point = point.to_right();
            neighbors.push(self.point_at(next_point.x, next_point.y));
        }

        if point.y > 0 {
            let next_point = point.to_bottom();
            neighbors.push(self.point_at(next_point.x, next_point.y));
        }

        if point.y + 1 < self.height {
            let next_point = point.to_top();
            neighbors.push(self.point_at(next_point.x, next_point.y));
        }

        neighbors
    }

    /// Get an iterator that finds all the neighbors
    fn neighbors(&self) -> Neighbors {
        let start_point = self.point_at(0, 0);

        Neighbors {
            matrix: self,
            current_point: start_point,
        }
    }

    fn basin(&self, low_point: Point) -> Basin {
        Basin {
            matrix: self,
            start_point: low_point,
        }
    }
}

/// An iterator over each point and its neighbors
struct Neighbors<'a> {
    matrix: &'a Matrix,
    current_point: Point,
}

impl Iterator for Neighbors<'_> {
    type Item = (Point, Vec<Point>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point.x == self.matrix.width {
            self.current_point.x = 0;
            self.current_point.y += 1;
        }

        if self.current_point.y == self.matrix.height {
            return None;
        }

        let current_cell = self
            .matrix
            .point_at(self.current_point.x, self.current_point.y);

        let neighbors = self
            .matrix
            .get_neighbor(self.current_point)
            .into_iter()
            .collect();

        self.current_point.x += 1;
        Some((current_cell, neighbors))
    }
}

struct Basin<'a> {
    matrix: &'a Matrix,
    start_point: Point,
}

impl Basin<'_> {
    fn compute_size(&self) -> u32 {
        // For the current `self.low_point`, find its neighbors, filter out those with an height bigger than nine,
        // then for each point find its neighbors again until no one is left.
        let mut basin_points: Vec<Point> = vec![self.start_point];
        let mut basins_queue: Vec<Point> = vec![self.start_point];

        while basins_queue.len() > 0 {
            let current_point = basins_queue.pop().unwrap();
            basin_points.push(current_point);

            let neighbors = self.matrix.get_neighbor(current_point);
            let valid_neighbors = neighbors
                .iter()
                .filter(|point| point.height < 9)
                .filter(|point| !basin_points.contains(&point))
                .collect::<Vec<_>>();

            basins_queue.extend(valid_neighbors.into_iter());
        }

        // Remove duplicate points and then get the size
        basin_points.into_iter().unique().count() as u32
    }
}

impl Solution for DayNine {
    type Output = u32;

    fn input() -> &'static str {
        include_str!("./inputs/9.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let matrix = Matrix::new(input);

        // Find all the low points and compute their sum
        matrix
            .neighbors()
            .into_iter()
            .filter(|(point, neighbors)| {
                neighbors
                    .iter()
                    .all(|neighbor| point.height < neighbor.height)
            })
            .map(|(point, _)| (point.height as u32) + 1)
            .sum()
    }

    fn solve_second(input: &str) -> Self::Output {
        let matrix = Matrix::new(input);

        // Find all the low points and their coordinates
        let min_points = matrix
            .neighbors()
            .into_iter()
            .filter(|(point, neighbors)| {
                neighbors
                    .iter()
                    .all(|neighbor| point.height < neighbor.height)
            })
            .map(|(point, _)| (point));

        // For each low point, find a basin, compute its size and then multiply them together
        min_points
            .map(|point| matrix.basin(point).compute_size())
            .sorted()
            .rev()
            .take(3)
            .product()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (516, 1023660)
    }
}
