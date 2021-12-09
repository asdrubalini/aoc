use itertools::Itertools;

use super::Solution;

pub struct DayEight;

#[derive(Debug, Clone)]
struct ScrambledDigit {
    signals: String,
    identified_digit: Option<u8>,
}

impl ScrambledDigit {
    fn set_identified_digit(&mut self, digit: u8) {
        assert_eq!(self.identified_digit, None);
        self.identified_digit = Some(digit);
    }

    fn is_found(&self) -> bool {
        self.identified_digit.is_some()
    }

    // Check if self has everything common with other
    fn has_everything_common(&self, other: &Self) -> bool {
        &self
            .signals
            .chars()
            .filter(|chr| other.signals.contains(*chr))
            .count()
            == &other.signals.len()
    }

    fn has_everything_common_minus_one(&self, other: &Self) -> bool {
        &self
            .signals
            .chars()
            .filter(|chr| other.signals.contains(*chr))
            .count()
            == &(other.signals.len() - 1)
    }
}

impl PartialEq for ScrambledDigit {
    fn eq(&self, other: &Self) -> bool {
        self.signals == other.signals
    }
}

impl From<&str> for ScrambledDigit {
    fn from(signals: &str) -> Self {
        let valid_signals = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let signals = signals
            .chars()
            .sorted() // sorting makes a bunch of stuff easier
            .filter(|chr| valid_signals.contains(chr))
            .collect();

        Self {
            signals,
            identified_digit: None,
        }
    }
}

struct Entry {
    observed_patterns: Vec<ScrambledDigit>,
    output_digits: Vec<ScrambledDigit>,
    found_digits: Vec<Option<ScrambledDigit>>,
}

impl Entry {
    fn new(observed_patterns: Vec<ScrambledDigit>, output_digits: Vec<ScrambledDigit>) -> Self {
        Self {
            observed_patterns,
            output_digits,
            found_digits: vec![None; 10],
        }
    }

    /// Easy digits are the digits that have a unique numbers of ON signals,
    /// such as 1, 4, 7 and 8.
    /// These digits can be easily identified by just checking their length.
    fn find_easy_digits(&mut self) {
        for digit in self.observed_patterns.iter_mut() {
            println!("{:?}", digit);

            match digit.signals.len() {
                2 => {
                    digit.set_identified_digit(1);
                    self.found_digits[1] = Some(digit.clone());
                }
                4 => {
                    digit.set_identified_digit(4);
                    self.found_digits[4] = Some(digit.clone());
                }
                3 => {
                    digit.set_identified_digit(7);
                    self.found_digits[7] = Some(digit.clone());
                }
                7 => {
                    digit.set_identified_digit(8);
                    self.found_digits[8] = Some(digit.clone());
                }
                _ => continue,
            }
        }

        println!("\n");
    }

    fn compute_output(&self) -> u32 {
        let ciao = self
            .output_digits
            .iter()
            .map(|d| {
                let corresponding_digit = self
                    .found_digits
                    .iter()
                    .flatten()
                    .filter(|found| *found == d)
                    .next()
                    .unwrap();

                corresponding_digit.identified_digit.unwrap().to_string()
            })
            .collect::<String>();

        ciao.parse::<u32>().unwrap()
    }

    fn find_other_digits(&mut self) {
        let mut three = self
            .observed_patterns
            .iter()
            .filter(|d| d.signals.len() == 5)
            .filter(|d| d.has_everything_common(&self.found_digits[7].as_ref().unwrap()))
            .next()
            .unwrap()
            .to_owned();

        three.set_identified_digit(3);
        self.observed_patterns.retain(|d| d != &three);
        self.found_digits[3] = Some(three);

        let mut nine = self
            .observed_patterns
            .iter()
            .filter(|d| d.signals.len() == 6)
            .filter(|d| d.has_everything_common(&self.found_digits[4].as_ref().unwrap()))
            .next()
            .unwrap()
            .to_owned();

        nine.set_identified_digit(9);
        self.observed_patterns.retain(|d| d != &nine);
        self.found_digits[9] = Some(nine);

        let mut five = self
            .observed_patterns
            .iter()
            .filter(|d| d.signals.len() == 5)
            .filter(|d| d.has_everything_common_minus_one(&self.found_digits[9].as_ref().unwrap()))
            .next()
            .unwrap()
            .to_owned();

        five.set_identified_digit(5);
        self.observed_patterns.retain(|d| d != &five);
        self.found_digits[5] = Some(five);

        let mut two = self
            .observed_patterns
            .iter()
            .filter(|d| d.signals.len() == 5)
            .next()
            .unwrap()
            .to_owned();

        two.set_identified_digit(2);
        self.observed_patterns.retain(|d| d != &two);
        self.found_digits[2] = Some(two);

        let mut zero = self
            .observed_patterns
            .iter()
            .filter(|d| d.signals.len() == 6)
            .filter(|d| d.has_everything_common(&self.found_digits[1].as_ref().unwrap()))
            .next()
            .unwrap()
            .to_owned();

        zero.set_identified_digit(0);
        self.observed_patterns.retain(|d| d != &zero);
        self.found_digits[0] = Some(zero);

        let mut six = self
            .observed_patterns
            .iter()
            .filter(|d| d.signals.len() != 2)
            .filter(|d| d.signals.len() != 3)
            .filter(|d| d.signals.len() != 4)
            .filter(|d| d.signals.len() != 7)
            .next()
            .unwrap()
            .to_owned();

        six.set_identified_digit(6);
        self.observed_patterns.retain(|d| d != &six);
        self.found_digits[6] = Some(six);
    }
}

impl DayEight {
    fn parse_input(input: &str) -> Vec<Entry> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut s = line.split('|');
                let observed_patterns = s
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.into())
                    .collect();
                let output_digits = s
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.into())
                    .collect();

                Entry::new(observed_patterns, output_digits)
            })
            .collect()
    }
}

impl Solution for DayEight {
    type Output = u32;

    fn input() -> &'static str {
        include_str!("./inputs/8.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let input = Self::parse_input(input);

        input
            .into_iter()
            .map(|mut entry| {
                entry.find_easy_digits();

                // Filter out only known digits
                let known_digits = entry
                    .observed_patterns
                    .iter()
                    .filter(|d| d.identified_digit.is_some())
                    .collect::<Vec<_>>();

                // Count how many times known digits appear in output digits
                // Could be simplified by only checking their length in each entry
                entry
                    .output_digits
                    .iter()
                    .filter(|output_digit| {
                        known_digits
                            .iter()
                            .filter(|known_digit| ***known_digit == **output_digit)
                            .count()
                            > 0
                    })
                    .count()
            })
            .sum::<usize>() as u32
    }

    fn solve_second(input: &str) -> Self::Output {
        let input = Self::parse_input(input);

        input
            .into_iter()
            .map(|mut e| {
                e.find_easy_digits();
                e.find_other_digits();
                e.compute_output()
            })
            .sum()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (278, 0)
    }
}
