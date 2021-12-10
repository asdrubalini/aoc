use super::Solution;

#[derive(Debug, PartialEq, Eq)]
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
    errors: Vec<ErrorKind>,
    original_string: String,
}

#[derive(Debug, PartialEq, Eq)]
enum ErrorKind {
    CloseBracketNotFound,
    IncorrectCloseBracket(BracketKind),
}

impl ChunksEntry {
    fn from_str(input: &str) -> Self {
        let tokens = Token::lex(input);

        let mut current_pos = 0;
        let mut nodes = vec![];
        let mut errors = vec![];

        loop {
            let (next_pos, node, next_errors) =
                Self::parse_tokens(&tokens, current_pos + 1, vec![]);
            current_pos = next_pos;
            errors.extend(next_errors.into_iter());

            match node {
                Some(node) => nodes.push(node),
                None => break,
            }
        }

        Self {
            brackets: nodes,
            errors,
            original_string: input.to_string(),
        }
    }

    fn parse_tokens(
        tokens: &[Token],
        start_pos: usize,
        mut errors: Vec<ErrorKind>,
    ) -> (usize, Option<ChunkNode>, Vec<ErrorKind>) {
        if start_pos + 1 > tokens.len() {
            return (start_pos, None, errors);
        }

        let start_token = tokens[start_pos];

        if start_token.is_close() {
            return (start_pos, None, errors);
        }

        let expected_close_token = start_token.to_close();
        let next_close_tokens = tokens
            .iter()
            .skip(start_pos)
            .filter(|token| token.is_close())
            .collect::<Vec<_>>();

        let next_close_token = next_close_tokens.get(0);

        if next_close_token.is_none() {
            errors.push(ErrorKind::CloseBracketNotFound);
        } else {
            let next_close_token = **next_close_token.unwrap();
            if next_close_token != expected_close_token {
                errors.push(ErrorKind::IncorrectCloseBracket(next_close_token.kind()));
            }
        }

        // if close_tokens.len() == 0 {
        // errors.push(ErrorKind::CloseBracketNotFound);
        // } else if close_tokens.len() % 2 == 0 {
        // errors.push(ErrorKind::MismatchedCloseBracket);
        // }

        let mut current_pos = start_pos;
        let mut current_node = ChunkNode {
            kind: start_token.kind(),
            inside: vec![],
        };

        loop {
            let (next_pos, node, next_errors) = Self::parse_tokens(tokens, current_pos + 1, errors);
            current_pos = next_pos;
            errors = next_errors;

            match node {
                Some(node) => current_node.inside.push(node),
                None => break,
            }
        }

        (current_pos, Some(current_node), errors)
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
    fn parse_input(input: &str) -> Vec<ChunksEntry> {
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
            let mismatched_errors_count = entry
                .errors
                .iter()
                .filter(|error| match **error {
                    ErrorKind::IncorrectCloseBracket(_) => true,
                    _ => false,
                })
                .count();

            println!("{} -> {:?}", entry.original_string, entry.errors);
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
