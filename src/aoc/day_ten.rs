use super::Solution;

#[derive(Debug)]
enum BracketKind {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Debug)]
struct ChunkNode {
    kind: BracketKind,
    inside: Vec<ChunkNode>,
}

#[derive(Debug)]
struct ChunksEntry {
    brackets: Vec<ChunkNode>,
    error_score: u32,
}

#[derive(Debug)]
enum ErrorKind {
    CloseBracketNotFound,
    MismatchedCloseBracket,
}

impl ChunksEntry {
    fn from_str(input: &str) -> Result<Self, ErrorKind> {
        let tokens = Token::lex(input);

        let mut current_pos = 0;
        let mut nodes = vec![];

        loop {
            let (next_pos, node) = Self::parse_tokens(&tokens, current_pos + 1)?;
            current_pos = next_pos;

            match node {
                Some(node) => nodes.push(node),
                None => break,
            }
        }

        Ok(Self {
            brackets: nodes,
            error_score: 0,
        })
    }

    fn parse_tokens(
        tokens: &[Token],
        start_pos: usize,
    ) -> Result<(usize, Option<ChunkNode>), ErrorKind> {
        if start_pos + 1 > tokens.len() {
            return Ok((start_pos, None));
        }

        let start_token = tokens[start_pos];

        if start_token.is_close() {
            return Ok((start_pos, None));
        }

        let close_token = start_token.to_close();
        let close_tokens = tokens
            .iter()
            .skip(start_pos)
            .filter(|token| token.is_close())
            .filter(|token| **token == close_token)
            .collect::<Vec<_>>();

        if close_tokens.len() == 0 {
            return Err(ErrorKind::CloseBracketNotFound);
        } else if close_tokens.len() % 2 == 0 {
            return Err(ErrorKind::MismatchedCloseBracket);
        }

        let mut current_pos = start_pos;
        let mut current_node = ChunkNode {
            kind: start_token.kind(),
            inside: vec![],
        };

        loop {
            let (next_pos, node) = Self::parse_tokens(tokens, current_pos + 1)?;
            current_pos = next_pos;

            match node {
                Some(node) => current_node.inside.push(node),
                None => break,
            }
        }

        Ok((current_pos, Some(current_node)))
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

    fn is_close(self) -> bool {
        match self {
            Token::RoundClose | Token::SquareClose | Token::CurlyClose | Token::AngleClose => true,
            _ => false,
        }
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

    fn kind(self) -> BracketKind {
        match self {
            Token::RoundOpen | Token::RoundClose => BracketKind::Round,
            Token::SquareOpen | Token::SquareClose => BracketKind::Square,
            Token::CurlyOpen | Token::CurlyClose => BracketKind::Curly,
            Token::AngleOpen | Token::AngleClose => BracketKind::Angle,
        }
    }
}

pub struct DayTen;

impl DayTen {
    fn parse_input(input: &str) -> Vec<Result<ChunksEntry, ErrorKind>> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(ChunksEntry::from_str)
            .collect()
    }
}

impl Solution for DayTen {
    type Output = u64;

    fn input() -> &'static str {
        include_str!("./inputs/example.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let input = Self::parse_input(input);

        for entry in input {
            match entry {
                Ok(entry) => println!(" ok"),
                Err(error) => {
                    println!(" {:?}", error);
                }
            }
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
