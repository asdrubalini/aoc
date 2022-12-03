use crate::aoc::Solution;
use itertools::Itertools;

pub struct Two;

#[derive(Debug, Clone, Copy)]
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
    fn score_against(&self, opponent_move: &Self) -> u32 {
        let score_match = match (self, opponent_move) {
            // draw
            (Move::Rock, Move::Rock) => 3,
            (Move::Paper, Move::Paper) => 3,
            (Move::Scissor, Move::Scissor) => 3,

            // lose
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

#[derive(Debug, Clone, Copy)]
enum RoundConclusion {
    Win,
    Lose,
    Draw,
}

impl From<Move> for RoundConclusion {
    fn from(from: Move) -> Self {
        match from {
            Move::Rock => Self::Lose,
            Move::Paper => Self::Draw,
            Move::Scissor => Self::Win,
        }
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
            .map(|(opponent_move, my_move)| my_move.score_against(opponent_move))
            .sum::<u32>()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        parsed
            .iter()
            .map(|(move_opponent, round_end)| {
                let round_end = RoundConclusion::from(*round_end);

                let my_move: Move = match (round_end, move_opponent) {
                    (RoundConclusion::Win, Move::Rock) => Move::Paper,
                    (RoundConclusion::Win, Move::Paper) => Move::Scissor,
                    (RoundConclusion::Win, Move::Scissor) => Move::Rock,

                    (RoundConclusion::Lose, Move::Rock) => Move::Scissor,
                    (RoundConclusion::Lose, Move::Paper) => Move::Rock,
                    (RoundConclusion::Lose, Move::Scissor) => Move::Paper,

                    (RoundConclusion::Draw, Move::Rock) => Move::Rock,
                    (RoundConclusion::Draw, Move::Paper) => Move::Paper,
                    (RoundConclusion::Draw, Move::Scissor) => Move::Scissor,
                };

                my_move.score_against(move_opponent)
            })
            .sum::<u32>()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (13446, 13509)
    }
}
