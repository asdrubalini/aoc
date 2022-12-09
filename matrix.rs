pub struct MatrixCell {
    visited_by_tail: bool,
}

impl Debug for MatrixCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.visited_by_tail { 0 } else { 1 })
    }
}

impl Default for MatrixCell {
    /// new empty
    fn default() -> Self {
        Self {
            visited_by_tail: false,
        }
    }
}

impl MatrixCell {
    fn new_origin() -> Self {
        MatrixCell {
            visited_by_tail: true,
        }
    }
}

#[derive(Debug)]
pub struct ExpandingMatrix {
    space: HashMap<Coord, MatrixCell>,
    edge_top_left: Coord,
    edge_bottom_right: Coord,
    head_position: Coord,
    tail_position: Coord,
}

impl Default for ExpandingMatrix {
    fn default() -> Self {
        let mut space = HashMap::new();
        space.insert(Coord(0, 0), MatrixCell::new_origin());

        Self {
            space,
            edge_top_left: Coord(-1, 1),
            edge_bottom_right: Coord(1, -1),
            head_position: Coord(0, 0),
            tail_position: Coord(0, 0),
        }
    }
}

impl ExpandingMatrix {
    fn at_mut(&mut self, x: i32, y: i32) -> &mut MatrixCell {
        let coord = Coord(x, y);

        if let Entry::Vacant(vacant) = self.space.entry(coord) {
            vacant.insert(MatrixCell::default());
        }

        self.space.get_mut(&coord).unwrap()
    }

    fn go_direction(&mut self, direction: Direction) {
        self.head_position.go_direction(direction);
    }

    fn debug_print(&self) {
        for y in (self.edge_bottom_right.y()..=self.edge_top_left.y()).rev() {
            for x in self.edge_top_left.x()..=self.edge_bottom_right.x() {
                let elem = self.space.get(&Coord(x, y)).unwrap();
                print!("{:?}", elem);
            }

            println!();
        }
    }
}
