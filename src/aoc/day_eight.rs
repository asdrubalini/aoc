use super::Solution;

pub struct DayEight;

impl DayEight {
    fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut s = line.split('|');
                let unique_patterns = s
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .collect();
                let output_digits = s
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .collect();

                (unique_patterns, output_digits)
            })
            .collect()
    }

    fn compare_digits_unoredered(this: &str, that: &str) -> bool {
        if this.len() != that.len() {
            return false;
        }

        this.chars().filter(|chr| that.contains(*chr)).count() == that.len()
    }
}

impl Solution for DayEight {
    type Output = u32;

    fn input() -> &'static str {
        include_str!("./inputs/8.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let input = Self::parse_input(input);

        let easy_lengths: Vec<usize> = vec![2, 4, 3, 7];

        let total_easy_digits_count = input
            .iter()
            .map(|(unique_patterns, output_digits)| {
                let easy_digits = unique_patterns
                    .iter()
                    .filter(|s| {
                        easy_lengths
                            .iter()
                            .filter(|length| s.len() == **length)
                            .count()
                            > 0
                    })
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>();

                let easy_digits_in_output = output_digits.iter().filter(|output_digit| {
                    easy_digits
                        .iter()
                        .filter(|easy_digit| {
                            Self::compare_digits_unoredered(output_digit, easy_digit)
                        })
                        .count()
                        > 0
                });

                easy_digits_in_output.count()
            })
            .sum::<usize>();

        total_easy_digits_count as u32
    }

    fn solve_second(input: &str) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (278, 0)
    }
}
