#![allow(dead_code)]
use std::{
    collections::{hash_map, HashMap},
    fmt::Debug,
};

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
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(Debug)]
pub struct InfiniteMatrix<T: Debug> {
    inner: HashMap<Coord, T>,

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
            inner: HashMap::new(),
            top_left_corner: Coord(0, 0),
            bottom_right_corner: Coord(0, 0),
        }
    }
}

impl<T: Debug> IntoIterator for InfiniteMatrix<T> {
    type Item = (Coord, T);
    type IntoIter = hash_map::IntoIter<Coord, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T: Debug> InfiniteMatrix<T> {
    pub fn new(value: T) -> Self {
        let mut m = InfiniteMatrix::default();

        m.set(Coord(0, 0), value);

        m
    }

    fn coord_is_in_matrix(&self, coord: Coord) -> bool {
        coord.x() > self.top_left_corner.x()
            && coord.x() < self.bottom_right_corner.x()
            && coord.y() > self.top_left_corner.y()
            && coord.y() < self.bottom_right_corner.y()
    }

    fn expand_corners_to_fit(&mut self, coord: Coord) {
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
}
