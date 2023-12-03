use itertools::Itertools;

use crate::aoc::Solution;

pub struct Two;

#[derive(Debug, Default)]
pub struct Set {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl From<&str> for Set {
    fn from(set_raw: &str) -> Self {
        let mut set = Self::default();
        let colors = set_raw.split(',').collect_vec();

        for color in colors {
            if color.contains("red") {
                let n = color.replace("red", "").parse().unwrap();
                set.red = Some(n);
            } else if color.contains("green") {
                let n = color.replace("green", "").parse().unwrap();
                set.green = Some(n);
            } else if color.contains("blue") {
                let n = color.replace("blue", "").parse().unwrap();
                set.blue = Some(n);
            } else {
                panic!("lol");
            };
        }

        set
    }
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let line = line.replace(' ', "");
        let (id, sets) = line.split(':').collect_tuple().unwrap();

        let id = id.replace("Game", "").parse().unwrap();

        let sets_vec = sets.split(';').collect_vec();
        let sets = sets_vec.into_iter().map(Set::from).collect_vec();

        Game { id, sets }
    }
}

impl Solution for Two {
    type Output = u32;
    type Parsed = Vec<Game>;

    fn input() -> &'static str {
        include_str!("../inputs/2.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Game::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        // Determine which games would have been possible if the bag had been loaded
        // with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum
        // of the IDs of those games?
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        parsed
            .iter()
            .filter_map(|game| {
                for set in game.sets.iter() {
                    if set.red.unwrap_or_default() > max_red
                        || set.green.unwrap_or_default() > max_green
                        || set.blue.unwrap_or_default() > max_blue
                    {
                        return None;
                    }
                }

                Some(game.id)
            })
            .sum()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .iter()
            .filter_map(|game| {
                let max_red = game
                    .sets
                    .iter()
                    .map(|s| s.red.unwrap_or_default())
                    .max()
                    .unwrap();
                let max_green = game
                    .sets
                    .iter()
                    .map(|s| s.green.unwrap_or_default())
                    .max()
                    .unwrap();
                let max_blue = game
                    .sets
                    .iter()
                    .map(|s| s.blue.unwrap_or_default())
                    .max()
                    .unwrap();

                Some(max_red * max_green * max_blue)
            })
            .sum()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (2563, 70768)
    }
}
