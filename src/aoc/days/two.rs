use crate::aoc::Solution;
use itertools::Itertools;

pub struct Two;

#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => panic!("unexpected {input}"),
        }
    }
}

impl Move {
    fn score_against(&self, move_opponent: &Self) -> u32 {
        let score_match = match (self, move_opponent) {
            // draw
            (Move::Rock, Move::Rock) => 3,
            (Move::Paper, Move::Paper) => 3,
            (Move::Scissor, Move::Scissor) => 3,

            // lost
            (Move::Rock, Move::Paper) => 0,
            (Move::Paper, Move::Scissor) => 0,
            (Move::Scissor, Move::Rock) => 0,

            // win
            (Move::Rock, Move::Scissor) => 6,
            (Move::Paper, Move::Rock) => 6,
            (Move::Scissor, Move::Paper) => 6,
        };

        let score_object = match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        };

        score_match + score_object
    }
}

impl Solution for Two {
    type Output = u32;
    type Parsed = Vec<(Move, Move)>;

    fn input() -> &'static str {
        include_str!("../inputs/2.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (opponent, me) = line.split(' ').collect_tuple().unwrap();
                (Move::from(opponent), Move::from(me))
            })
            .collect()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .iter()
            .map(|(move_opponent, move_me)| move_me.score_against(move_opponent))
            .sum::<u32>()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (13446, 0)
    }
}
