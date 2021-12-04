use super::Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Board {
    inner: Vec<Vec<u8>>,
    winner_score: Option<u32>,
}

impl Board {
    fn new(input: &str) -> Self {
        let inner = input
            .lines()
            .map(|line| {
                line.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|row| !row.is_empty())
            .collect::<Vec<_>>();

        Self {
            inner,
            winner_score: None,
        }
    }

    fn check_win(&self, drawn_numbers: &[u8]) -> bool {
        // check horizontally
        for row in &self.inner {
            // check for match
            if row.iter().all(|elem| drawn_numbers.contains(elem)) {
                return true;
            }
        }

        // check vertically
        for i in 0..self.inner.len() {
            let mut col = self.inner.iter().map(|row| row.get(i).unwrap());
            // check for match
            if col.all(|elem| drawn_numbers.contains(elem)) {
                return true;
            }
        }

        false
    }

    fn compute_score(&mut self, drawn_numbers: &[u8]) {
        let unmarked_sum: u32 = self
            .inner
            .iter()
            .flatten()
            .filter(|item| !drawn_numbers.contains(item))
            .map(|n| *n as u32)
            .sum();

        let score = unmarked_sum * (*drawn_numbers.last().unwrap() as u32);
        self.winner_score = Some(score);
    }
}

pub struct DayFour;

impl DayFour {
    fn parse_drawn_numbers(input: &str) -> Vec<u8> {
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect()
    }

    fn parse_boards(input: &str) -> Vec<Board> {
        let boards_raw = input.split("\n\n").skip(1);
        boards_raw.map(Board::new).collect()
    }

    fn get_ordered_winners(input: &str) -> Vec<Board> {
        let numbers = Self::parse_drawn_numbers(input);
        let mut boards = Self::parse_boards(input);

        let mut winners = vec![];

        for i in 0..numbers.len() {
            let drawn_numbers = numbers.get(0..=i).unwrap();

            let current_winners = boards
                .clone()
                .iter()
                .filter(|b| b.check_win(drawn_numbers))
                .map(|b| b.to_owned())
                .collect::<Vec<_>>();

            for winner in current_winners.iter() {
                let mut winner = winner.clone();
                winner.compute_score(drawn_numbers);
                winners.push(winner);
            }

            boards = boards
                .iter()
                .filter(|board| !current_winners.contains(board))
                .map(|b| b.to_owned())
                .collect();
        }

        winners
    }
}

impl Solution for DayFour {
    type Output = u32;

    fn input() -> &'static str {
        include_str!("./inputs/4.txt")
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        Self::get_ordered_winners(input)
            .get(0)
            .unwrap()
            .winner_score
            .unwrap()
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        Self::get_ordered_winners(input)
            .last()
            .unwrap()
            .winner_score
            .unwrap()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (16674, 7075)
    }
}
