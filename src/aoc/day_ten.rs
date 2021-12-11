use itertools::Itertools;

use super::Solution;

struct Entry {
    tokens: Vec<Token>,
}

impl Entry {
    fn new(line: &str) -> Self {
        let tokens = Token::lex(line);
        Self { tokens }
    }

    fn has_apparent_corruption(&self) -> bool {
        false
    }

    fn find_corruption(&mut self) {
        loop {
            let pop_idx = self
                .tokens
                .iter()
                .enumerate()
                .tuple_windows()
                .filter(|(prev, next)| prev.1.to_close() == *next.1)
                .map(|(prev, next)| (prev.0, next.0))
                .next();

            match pop_idx {
                Some((prev, next)) => {
                    self.tokens.remove(prev);
                    self.tokens.remove(prev);
                }
                None => break,
            };
        }

        println!("{:?}", self.tokens);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngleOpen,
    AngleClose,
}

impl Token {
    /// An `input` is in the form of
    /// [({(<(())[]>[[{[]{<()<>>
    /// this method converts a string into a Vec of Tokens,
    /// panicking if an unexpected token is encountered
    fn lex(input: &str) -> Vec<Token> {
        input
            .chars()
            .map(|chr| match chr {
                '(' => Token::RoundOpen,
                ')' => Token::RoundClose,
                '[' => Token::SquareOpen,
                ']' => Token::SquareClose,
                '{' => Token::CurlyOpen,
                '}' => Token::CurlyClose,
                '<' => Token::AngleOpen,
                '>' => Token::AngleClose,
                _ => panic!("invalid token"),
            })
            .collect()
    }

    fn to_close(self) -> Self {
        match self {
            Token::RoundOpen => Token::RoundClose,
            Token::SquareOpen => Token::SquareClose,
            Token::CurlyOpen => Token::CurlyClose,
            Token::AngleOpen => Token::AngleClose,
            _ => panic!("Cannot convert open Token to close"),
        }
    }
}

pub struct DayTen;

impl DayTen {
    fn parse_input(input: &str) -> Vec<Entry> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(Entry::new)
            .collect()
    }
}

impl Solution for DayTen {
    type Output = u64;

    fn input() -> &'static str {
        include_str!("./inputs/example.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let mut entries = Self::parse_input(input);

        for entry in entries.iter_mut() {
            entry.find_corruption();
            break;
        }

        0
    }

    fn solve_second(input: &str) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
