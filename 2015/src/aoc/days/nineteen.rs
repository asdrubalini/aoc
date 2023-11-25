use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
};

use itertools::Itertools;
use petgraph::{
    dot::{Config, Dot},
    graph::NodeIndex,
    Graph,
};

use crate::aoc::Solution;

pub struct Nineteen;

type Molecule = Vec<char>;

trait MoleculeString {
    fn to_string(&self) -> String;
}

impl MoleculeString for Molecule {
    fn to_string(&self) -> String {
        self.iter().collect()
    }
}

fn debug_molecule(molecule: impl AsRef<Molecule>) {
    let molecule = molecule.as_ref();

    println!("{}", molecule.to_string());
}

#[derive(Debug, Clone)]
pub struct Replacement {
    from: Molecule,
    to: Molecule,
}

impl From<&str> for Replacement {
    fn from(line: &str) -> Self {
        let mut line = line.split(' ');

        let from = line.next().unwrap().chars().collect_vec();
        line.next().unwrap();
        let to = line.next().unwrap().chars().collect_vec();

        Replacement { from, to }
    }
}

impl Replacement {
    fn molecule_process_all_combinations(&self, molecule: &Molecule) -> Vec<Molecule> {
        let mut results = vec![];

        let windows = molecule.windows(self.from.len()).enumerate();

        for (i, chars) in windows {
            if chars != self.from {
                continue;
            }

            let mut s = molecule.iter().copied().take(i).collect_vec();
            s.extend(self.to.iter());
            s.extend(molecule.iter().skip(i + chars.len()));

            results.push(s);
        }

        results
    }
}

#[derive(Debug, Clone)]
pub struct NuclearPlant {
    replacements: Vec<Replacement>,
    molecules_nodes: HashMap<Molecule, NodeIndex>,
    target_molecule: Molecule,
}

impl NuclearPlant {
    fn molecule_upsert_node(
        &mut self,
        g: &mut Graph<String, ()>,
        molecule: &Molecule,
    ) -> NodeIndex {
        self.molecules_nodes
            .entry(molecule.to_owned())
            .or_insert(g.add_node(molecule.to_string()))
            .to_owned()
    }

    fn build_graph(&mut self, starting_molecule: &Molecule) -> Graph<String, ()> {
        let mut g = Graph::<String, ()>::new();

        let starting_molecule_node = self.molecule_upsert_node(&mut g, starting_molecule);

        let mut queue: VecDeque<(Molecule, NodeIndex)> = VecDeque::new();
        queue.push_back((starting_molecule.to_owned(), starting_molecule_node));

        let mut count = 0;

        while let Some((molecule, node)) = queue.pop_front() {
            // debug_molecule(&b_molecule);
            dbg!(queue.iter().count());

            let replacement_results = self
                .replacements
                .iter()
                .map(|replacement| replacement.molecule_process_all_combinations(&molecule))
                .flatten()
                .unique()
                .collect_vec();

            for result in replacement_results {
                let result_index = self.molecule_upsert_node(&mut g, &result);
                g.add_edge(node, result_index, ());

                if result == self.target_molecule {
                    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
                    panic!("done {count}");
                }

                let c = (result, result_index);

                if !queue.contains(&c) {
                    queue.push_back(c);
                }
            }

            count += 1;

            if count == 10 {
                println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

                // dbg!(&queue);
                panic!();
            }
        }

        g
    }
}

impl Solution for Nineteen {
    type Output = u32;
    type Parsed = NuclearPlant;

    fn input() -> &'static str {
        include_str!("../inputs/19.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        let lines = input.lines().collect_vec();
        let (separator_idx, _) = lines.iter().find_position(|l| l.is_empty()).unwrap();

        let replacements = lines
            .iter()
            .take(separator_idx)
            .map(|l| Replacement::from(*l))
            .collect_vec();

        let molecule = lines.get(separator_idx + 1).unwrap().chars().collect_vec();

        NuclearPlant {
            replacements,
            target_molecule: molecule,
            molecules_nodes: HashMap::new(),
        }
    }

    fn solve_first(nuclear_plant: &Self::Parsed) -> Self::Output {
        let mut seen_combinations: HashSet<Molecule> = HashSet::new();

        for replacement in &nuclear_plant.replacements {
            for result in
                replacement.molecule_process_all_combinations(&nuclear_plant.target_molecule)
            {
                seen_combinations.insert(result);
            }
        }

        seen_combinations.len() as u32
    }

    fn solve_second(nuclear_plant: &Self::Parsed) -> Self::Output {
        let mut nuclear_plant = nuclear_plant.to_owned();

        let g = nuclear_plant.build_graph(&vec!['e']);

        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (535, 0)
    }
}
