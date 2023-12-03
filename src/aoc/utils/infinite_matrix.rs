#![allow(dead_code)]

use core::panic;
use std::{
    collections::{hash_map, HashMap},
    fmt::Debug,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.0
    }

    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.1
    }

    pub fn neighbor_west(&self) -> Self {
        let mut c = *self;
        *c.x_mut() -= 1;
        c
    }

    pub fn neighbor_east(&self) -> Self {
        let mut c = *self;
        *c.x_mut() += 1;
        c
    }

    pub fn neighbor_north(&self) -> Self {
        let mut c = *self;
        *c.y_mut() += 1;
        c
    }

    pub fn neighbor_south(&self) -> Self {
        let mut c = *self;
        *c.y_mut() -= 1;
        c
    }

    pub fn neighbor_north_west(&self) -> Self {
        self.neighbor_north().neighbor_west()
    }

    pub fn neighbor_north_east(&self) -> Self {
        self.neighbor_north().neighbor_east()
    }

    pub fn neighbor_south_west(&self) -> Self {
        self.neighbor_south().neighbor_west()
    }

    pub fn neighbor_south_east(&self) -> Self {
        self.neighbor_south().neighbor_east()
    }

    pub fn all_neighbors_with_diagonal(&self) -> Vec<Self> {
        vec![
            self.neighbor_east(),
            self.neighbor_north_east(),
            self.neighbor_north(),
            self.neighbor_north_west(),
            self.neighbor_west(),
            self.neighbor_south_west(),
            self.neighbor_south(),
            self.neighbor_south_east(),
        ]
    }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

/// Parse "123,321"
impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let (x, y) = value.split(',').collect_tuple().unwrap();

        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();

        Coord(x, y)
    }
}

#[derive(Debug, Clone)]
pub struct InfiniteMatrix<T: Debug> {
    inner: HashMap<Coord, T>,

    fixed: bool,

    // top_left_corner
    // +---------------+
    // |       |       |
    // |       |       |
    // |-------+---–---|   <== (0, 0) at the center
    // |       |       |
    // |       |       |
    // +---------------+
    //             bottom_right_corner
    top_left_corner: Coord,
    bottom_right_corner: Coord,
}

impl<T: Debug> Default for InfiniteMatrix<T> {
    fn default() -> Self {
        Self {
            fixed: false,
            inner: HashMap::new(),
            top_left_corner: Coord(0, 0),
            bottom_right_corner: Coord(0, 0),
        }
    }
}

impl<T: Debug + Default> IntoIterator for InfiniteMatrix<T> {
    type Item = (Coord, T);
    type IntoIter = hash_map::IntoIter<Coord, T>;

    fn into_iter(mut self) -> Self::IntoIter {
        // Insert all the missing values first
        self.allocate_all();
        self.inner.into_iter()
    }
}

impl<T: Debug> InfiniteMatrix<T> {
    pub fn new(value: T) -> Self {
        let mut m = InfiniteMatrix::default();

        m.set(Coord(0, 0), value);

        m
    }

    pub fn cols_count(&self) -> usize {
        panic!("untested");
        (self.top_left_corner.x() + 1).abs_diff(self.bottom_right_corner.x() - 1) as usize
    }

    pub fn rows_count(&self) -> usize {
        panic!("untested");
        (self.top_left_corner.y() - 1).abs_diff(self.bottom_right_corner.y() + 1) as usize
    }

    pub fn new_fixed(width: usize, height: usize) -> Self {
        InfiniteMatrix {
            inner: HashMap::default(),
            top_left_corner: Coord(-1, height as i32),
            bottom_right_corner: Coord(width as i32, -1),
            fixed: true,
        }
    }

    pub fn from_other_matrix_config<A: Debug>(other: &InfiniteMatrix<A>) -> Self {
        InfiniteMatrix {
            inner: HashMap::default(),
            top_left_corner: other.top_left_corner,
            bottom_right_corner: other.bottom_right_corner,
            fixed: other.fixed,
        }
    }

    pub fn set_fixed(&mut self, fixed: bool) {
        self.fixed = fixed;
    }

    fn coord_is_in_matrix(&self, coord: Coord) -> bool {
        coord.x() > self.top_left_corner.x()
            && coord.x() < self.bottom_right_corner.x()
            && coord.y() < self.top_left_corner.y()
            && coord.y() > self.bottom_right_corner.y()
    }

    fn expand_corners_to_fit(&mut self, coord: Coord) {
        if self.fixed {
            panic!(
                "Trying to expand a fixed Matrix: {coord:?} while corners are {:?}, {:?}",
                self.top_left_corner, self.bottom_right_corner
            );
        }

        if coord.x() <= self.top_left_corner.x() {
            *self.top_left_corner.x_mut() = coord.x() - 1;
        }

        if coord.x() >= self.bottom_right_corner.x() {
            *self.bottom_right_corner.x_mut() = coord.x() + 1;
        }

        if coord.y() <= self.bottom_right_corner.y() {
            *self.bottom_right_corner.y_mut() = coord.y() - 1;
        }

        if coord.y() >= self.top_left_corner.y() {
            *self.top_left_corner.y_mut() = coord.y() + 1;
        }
    }

    pub fn at(&self, coord: Coord) -> Option<&T> {
        if !self.coord_is_in_matrix(coord) {
            return None;
        }

        self.inner.get(&coord)
    }

    pub fn at_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if !self.coord_is_in_matrix(coord) {
            self.expand_corners_to_fit(coord);
        }

        self.inner.get_mut(&coord)
    }

    pub fn entry(&mut self, coord: Coord) -> hash_map::Entry<'_, Coord, T> {
        self.inner.entry(coord)
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        match self.at_mut(coord) {
            Some(cell) => {
                *cell = value;
            }

            None => {
                self.inner.insert(coord, value);
            }
        }
    }

    pub fn coords_iterator(&self) -> SpaceCoordsIterator {
        SpaceCoordsIterator::new(self.top_left_corner, self.bottom_right_corner)
    }

    pub fn rows_iterator(&self) -> RowColsIterator {
        RowColsIterator::new_rows(self.top_left_corner, self.bottom_right_corner)
    }

    /*
    pub fn cols_iterator(&self) -> RowColsIterator {
        RowColsIterator::new_cols(self.top_left_corner, self.bottom_right_corner)
    }
    */
}

impl<T: Debug> From<Vec<Vec<T>>> for InfiniteMatrix<T> {
    fn from(input: Vec<Vec<T>>) -> Self {
        let mut matrix = InfiniteMatrix::<T>::new_fixed(input.first().unwrap().len(), input.len());

        for (y, line) in input.into_iter().rev().enumerate() {
            for (x, item) in line.into_iter().enumerate() {
                matrix.set(Coord(x as i32, y as i32), item);
            }
        }

        matrix
    }
}

impl<T: Debug + Default> InfiniteMatrix<T> {
    fn allocate_all(&mut self) {
        for coord in self.coords_iterator() {
            assert!(self.coord_is_in_matrix(coord));

            self.entry(coord).or_default();
        }
    }
}

pub struct SpaceCoordsIterator {
    iter: Box<dyn Iterator<Item = Coord>>,
}

fn get_boundaries_inclusive(
    top_left_corner: Coord,
    bottom_right_corner: Coord,
) -> (i32, i32, i32, i32) {
    let (start_x, start_y) = (top_left_corner.x() + 1, top_left_corner.y() - 1);
    let (end_x, end_y) = (bottom_right_corner.x() - 1, bottom_right_corner.y() + 1);

    (start_x, start_y, end_x, end_y)
}

impl SpaceCoordsIterator {
    fn new(top_left_corner: Coord, bottom_right_corner: Coord) -> Self {
        let (start_x, start_y, end_x, end_y) =
            get_boundaries_inclusive(top_left_corner, bottom_right_corner);

        let inner = (end_y..=start_y)
            .rev()
            .flat_map(move |y| (start_x..=end_x).map(move |x| Coord(x, y)));

        SpaceCoordsIterator {
            iter: Box::new(inner),
        }
    }
}

impl Iterator for SpaceCoordsIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct RowColsIterator {
    iter: Box<dyn Iterator<Item = Vec<Coord>>>,
}

impl RowColsIterator {
    fn new_rows(top_left_corner: Coord, bottom_right_corner: Coord) -> Self {
        let (start_x, start_y, end_x, end_y) =
            get_boundaries_inclusive(top_left_corner, bottom_right_corner);

        let inner = (end_y..=start_y)
            .rev()
            .map(move |y| (start_x..=end_x).map(move |x| Coord(x, y)).collect_vec());

        RowColsIterator {
            iter: Box::new(inner),
        }
    }

    /*
    fn new_cols(top_left_corner: Coord, bottom_right_corner: Coord) -> Self {
        let (start_x, start_y, end_x, end_y) =
            get_boundaries_inclusive(top_left_corner, bottom_right_corner);

        let inner = (end_x..=start_x)
            .rev()
            .map(move |y| (end_y..=start_y).map(move |x| Coord(x, y)).collect_vec());

        RowColsIterator {
            iter: Box::new(inner),
        }
    }
    */
}

impl Iterator for RowColsIterator {
    type Item = Vec<Coord>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::utils::Coord;

    use super::InfiniteMatrix;

    #[test]
    fn test_empty_matrix() {
        let m = InfiniteMatrix::<()>::default();

        assert_eq!(m.top_left_corner, Coord(0, 0));
        assert_eq!(m.bottom_right_corner, Coord(0, 0));
    }

    #[test]
    fn test_expand_matrix_singe() {
        let mut m = InfiniteMatrix::<()>::default();
        m.expand_corners_to_fit(Coord(0, 0));

        assert_eq!(m.top_left_corner, Coord(-1, 1));
        assert_eq!(m.bottom_right_corner, Coord(1, -1));
    }

    #[test]
    fn test_expand_matrix_col() {
        let mut m = InfiniteMatrix::<()>::default();
        m.expand_corners_to_fit(Coord(10, 10));

        assert_eq!(m.top_left_corner, Coord(0, 11));
        assert_eq!(m.bottom_right_corner, Coord(11, 0));
    }

    #[test]
    fn test_expand_rectangle() {
        let mut m = InfiniteMatrix::<()>::default();
        m.expand_corners_to_fit(Coord(10, 10));
        m.expand_corners_to_fit(Coord(-10, -10));

        assert_eq!(m.top_left_corner, Coord(-11, 11));
        assert_eq!(m.bottom_right_corner, Coord(11, -11));
    }

    #[test]
    fn test_fixed_matrix() {
        let mut m = InfiniteMatrix::<()>::new_fixed(10, 10);
        m.set(Coord(0, 0), ());
    }

    #[test]
    fn test_coords_iterator() {
        let m = InfiniteMatrix::<()>::new_fixed(10, 10);
        assert_eq!(m.coords_iterator().count(), m.into_iter().count());
    }

    /*
    #[test]
    fn test_rows_iterator_count() {
        let m = InfiniteMatrix::<()>::new_fixed(10, 10);
        assert_eq!(m.rows_iterator().count(), m.rows_count());
    }
    */
}
