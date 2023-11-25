#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{algo::min_spanning_tree, graph::NodeIndex, prelude::UnGraph};

use crate::aoc::Solution;

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
    nodes: HashMap<String, NodeIndex>,
    pub graph: UnGraph<String, u16>,
}

impl<'a> FromIterator<&'a str> for Connections {
    fn from_iter<T: IntoIterator<Item = &'a str>>(lines: T) -> Self {
        let mut nodes = HashMap::new();
        let mut graph = UnGraph::new_undirected();

        for (from, to, distance) in lines.into_iter().map(Connection::parse) {
            let from_node = nodes
                .entry(from.clone())
                .or_insert_with(|| graph.add_node(from))
                .to_owned();

            let to_node = nodes
                .entry(to.clone())
                .or_insert_with(|| graph.add_node(to))
                .to_owned();

            graph.add_edge(from_node, to_node, distance);
        }

        Connections { nodes, graph }
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
        let tree = min_spanning_tree(&parsed.graph);

        tree.into_iter()
            .filter_map(|e| match e {
                petgraph::data::Element::Node { weight: _ } => None,
                petgraph::data::Element::Edge {
                    source: _,
                    target: _,
                    weight,
                } => Some(weight as u32),
            })
            .sum()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
