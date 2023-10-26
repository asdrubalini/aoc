use std::collections::HashSet;

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Nineteen;

#[derive(Debug)]
pub struct Replacement {
    from: Vec<char>,
    to: Vec<char>,
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

#[derive(Debug)]
pub struct NuclearPlant {
    replacements: Vec<Replacement>,
    molecule: Vec<char>,
}

impl NuclearPlant {
    fn molecule_process(&self, replacement: &Replacement) -> Vec<Vec<char>> {
        let mut results = vec![];

        let mut windows = self.molecule.windows(replacement.from.len()).enumerate();

        while let Some((i, chars)) = windows.next() {
            if chars != replacement.from {
                continue;
            }

            let mut s = self.molecule.iter().copied().take(i).collect_vec();
            s.extend(replacement.to.iter());
            s.extend(self.molecule.iter().skip(i + chars.len()));

            results.push(s);
        }

        results
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
            molecule,
        }
    }

    fn solve_first(nuclear_plant: &Self::Parsed) -> Self::Output {
        let mut seen_combinations: HashSet<Vec<char>> = HashSet::new();

        for replacement in &nuclear_plant.replacements {
            for result in nuclear_plant.molecule_process(replacement) {
                if !seen_combinations.contains(&result) {}
                seen_combinations.insert(result);
            }
        }

        seen_combinations.len() as u32
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (535, 0)
    }
}
