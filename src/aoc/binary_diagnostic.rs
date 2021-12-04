use super::Solution;

pub struct DayThree;

impl DayThree {
    fn get_bins(input: String) -> Vec<Vec<char>> {
        input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|c| c.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>()
    }

    fn filter_out_bits(
        mut candidates: Vec<Vec<char>>,
        filter: fn(bit: char, one_count: usize, zero_count: usize) -> bool,
    ) -> u64 {
        let mut current_bit = 0;

        while candidates.len() > 1 {
            let one_bits_count = candidates
                .iter()
                .map(|bits| bits.get(current_bit).unwrap())
                .filter(|bit| bit == &&'1')
                .count();

            let zero_bits_count = candidates
                .iter()
                .map(|bits| bits.get(current_bit).unwrap())
                .filter(|bit| bit == &&'0')
                .count();

            candidates = candidates
                .iter_mut()
                .filter(|bits| {
                    let bit = bits.get(current_bit).unwrap();
                    filter(*bit, one_bits_count, zero_bits_count)
                })
                .map(|bits| bits.to_owned())
                .collect();

            current_bit += 1;
        }

        let rating = candidates.get_mut(0).unwrap().iter().collect::<String>();
        u64::from_str_radix(&rating, 2).unwrap()
    }
}

impl Solution for DayThree {
    type Output = u64;

    fn input() -> String {
        include_str!("./inputs/3.txt").to_string()
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();
        let bins = Self::get_bins(input.to_string());

        let bins_length = bins.len();
        let mut ones_count = vec![0; bins[0].len()];

        for bin in bins {
            for (i, bit) in bin.iter().enumerate() {
                if bit == &'1' {
                    (*ones_count.get_mut(i).unwrap()) += 1;
                }
            }
        }

        let gamma_rate = ones_count
            .iter()
            .map(|one_count| {
                if one_count >= &(bins_length / 2) {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>();

        let epsilon_rate = gamma_rate
            .chars()
            .map(|c| match c {
                '0' => '1',
                '1' => '0',
                _ => panic!("Wrong digit"),
            })
            .collect::<String>();

        let gamma_rate = u64::from_str_radix(&gamma_rate, 2).unwrap();
        let epsilon_rate = u64::from_str_radix(&epsilon_rate, 2).unwrap();

        gamma_rate * epsilon_rate
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();
        let bins = Self::get_bins(input.to_string());

        let oxygen_generator_rating =
            Self::filter_out_bits(bins.clone(), |bit, one_count, zero_count| {
                bit == if one_count >= zero_count { '1' } else { '0' }
            });

        let scrubber_rating = Self::filter_out_bits(bins, |bit, one_count, zero_count| {
            bit == if zero_count <= one_count { '0' } else { '1' }
        });

        oxygen_generator_rating * scrubber_rating
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (852500, 1007985)
    }
}
