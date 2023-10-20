use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate(i32, i32);

impl Coordinate {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.0
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.1
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

pub struct InfiniteMatrix<T: Debug> {
    inner: HashMap<Coordinate, T>,

    /// top left
    /// +---------------+
    /// |               |
    /// |               |
    /// |     (x,y)     |
    /// |               |
    /// |               |
    /// +---------------+
    ///             bottom right
    top_left_corner: Coordinate,
    bottom_right_corner: Coordinate,
}

impl<T: Debug> Default for InfiniteMatrix<T> {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
            top_left_corner: Coordinate(0, 0),
            bottom_right_corner: Coordinate(0, 0),
        }
    }
}

impl<T: Debug> InfiniteMatrix<T> {
    pub fn coord_in_matrix(&self, coord: Coordinate) -> bool {
        coord.x() > self.top_left_corner.x()
            && coord.x() < self.bottom_right_corner.x()
            && coord.y() > self.top_left_corner.y()
            && coord.y() < self.bottom_right_corner.y()
    }

    pub fn expand_corners_to_fit(&mut self, coord: Coordinate) {
        if coord.x() < self.top_left_corner.x() + 1 {
            *self.top_left_corner.x_mut() = coord.x() - 1;
        }

        if coord.x() > self.bottom_right_corner.x() - 1 {
            *self.bottom_right_corner.x_mut() = coord.x() + 1;
        }

        if coord.y() < self.top_left_corner.y() + 1 {
            *self.top_left_corner.y_mut() = coord.y() - 1;
        }

        if coord.y() > self.bottom_right_corner.y() - 1 {
            *self.bottom_right_corner.y_mut() = coord.y() + 1;
        }
    }

    pub fn area(&self) -> u32 {
        self.top_left_corner
            .x()
            .abs_diff(self.bottom_right_corner.x() - 1)
            * self
                .top_left_corner
                .y()
                .abs_diff(self.bottom_right_corner.y() - 1)
    }

    pub fn at(&self, x: i32, y: i32) -> Result<Option<&T>, ()> {
        let coord: Coordinate = (x, y).into();

        if !self.coord_in_matrix(coord) {
            // panic!("Coord is not in matrix");
            return Err(());
        }

        // TODO: return None if cell is empty, panic if cell doesnt exist
        Ok(self.inner.get(&coord))
    }

    pub fn set(&mut self, x: i32, y: i32, value: T) {
        let coord: Coordinate = (x, y).into();

        match self.inner.entry(coord) {
            std::collections::hash_map::Entry::Occupied(mut occupied) => {
                // Replace existing value
                occupied.insert(value);
            }

            std::collections::hash_map::Entry::Vacant(vacant) => {
                vacant.insert(value);

                // Expand matrix if necessary
                if !self.coord_in_matrix(coord) {
                    self.expand_corners_to_fit(coord);
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::InfiniteMatrix;

    #[test]
    fn test_empty_matrix() {
        let m = InfiniteMatrix::<()>::default();
        assert_eq!(m.area(), 0);
    }

    #[test]
    fn test_expand_matrix_singe() {
        let mut m = InfiniteMatrix::<()>::default();
        m.set(0, 0, ());

        assert_eq!(m.area(), 1);
    }

    #[test]
    fn test_expand_matrix_col() {
        let mut m = InfiniteMatrix::<()>::default();
        m.set(0, 10, ());

        assert_eq!(m.area(), 10);
    }
}
