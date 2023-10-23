use std::vec;

use itertools::Itertools;
use petgraph::{algo::min_spanning_tree, prelude::UnGraph};
use petgraph_evcxr::draw_graph;

use crate::aoc::{utils::UniqueIndexMap, Solution};

#[derive(Debug, Clone, Copy)]
pub struct Connection {
    from: u32,
    to: u32,

    distance: u16,
}

impl Connection {
    fn parse(line: &str) -> (String, String, u16) {
        let x = line.split(' ').collect_vec();

        let from = x.first().unwrap().to_string();
        let to = x.get(2).unwrap().to_string();
        let distance = x.get(4).unwrap().parse().unwrap();

        (from, to, distance)
    }

    fn new(from: u32, to: u32, distance: u16) -> Self {
        Connection { from, to, distance }
    }
}

#[derive(Debug)]
pub struct Connections {
    inner: Vec<Connection>,
    ids: UniqueIndexMap<String>,
}

impl<'a> FromIterator<&'a str> for Connections {
    fn from_iter<T: IntoIterator<Item = &'a str>>(lines: T) -> Self {
        let mut ids = UniqueIndexMap::default();

        let connections = lines
            .into_iter()
            .map(Connection::parse)
            .map(|(from, to, distance)| {
                let from = ids.obtain_id(from);
                let to = ids.obtain_id(to);

                Connection::new(from, to, distance)
            })
            .collect_vec();

        Connections {
            inner: connections,
            ids,
        }
    }
}

impl IntoIterator for &Connections {
    type Item = Connection;
    type IntoIter = vec::IntoIter<Connection>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.to_owned().into_iter()
    }
}

pub struct Nine;

impl Solution for Nine {
    type Output = u32;
    type Parsed = Connections;

    fn input() -> &'static str {
        include_str!("../inputs/9.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        Connections::from_iter(input.lines())
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let data = parsed
            .into_iter()
            .map(|connection| (connection.from, connection.to, connection.distance));

        let mut graph = UnGraph::<u32, u16>::from_edges(data);

        let ciao = min_spanning_tree(&graph);

        let x = ciao.into_iter();

        0
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
