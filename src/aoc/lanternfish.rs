use std::mem::replace;

use super::Solution;

#[derive(Debug)]
struct LanternfishGroup {
    counter: u32,
    fishes: Vec<u64>,
}

impl LanternfishGroup {
    fn new(timers: &[u8]) -> Self {
        let mut fishes = vec![0; 9];

        for timer in timers {
            let counter = fishes.get_mut(*timer as usize).unwrap();
            *counter += 1;
        }

        Self { counter: 0, fishes }
    }

    fn advance_all(&mut self) {
        self.counter += 1;
        dbg!(self.counter);

        let fishes = replace(&mut self.fishes, vec![0; 9]);

        for (timer, count) in fishes.into_iter().enumerate() {
            println!("{} -> {}", timer, count);

            match timer {
                0 => {
                    let counter = self.fishes.get_mut(6).unwrap();
                    *counter += count;

                    let counter = self.fishes.get_mut(8).unwrap();
                    *counter += count;
                }

                n => {
                    let counter = self.fishes.get_mut(n - 1).unwrap();
                    *counter += count;
                }
            }
        }

        // dbg!(&self.fishes);
        println!();
    }
}

impl Iterator for LanternfishGroup {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_all();
        Some(self.fishes.iter().map(|n| *n).sum())
    }
}

pub struct DaySix;

impl DaySix {
    fn parse_input(input: &str) -> Vec<u8> {
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect()
    }
}

impl Solution for DaySix {
    type Output = u64;

    fn input() -> &'static str {
        include_str!("./inputs/6.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let input = Self::parse_input(input);
        let mut group = LanternfishGroup::new(&input);

        group.nth(80 - 1).unwrap()
    }

    fn solve_second(input: &str) -> Self::Output {
        let input = Self::parse_input(input);
        let mut group = LanternfishGroup::new(&input);

        group.nth(256 - 1).unwrap()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (351188, 1595779846729)
    }
}
