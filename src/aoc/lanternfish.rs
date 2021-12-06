use super::Solution;

struct Lanternfish {
    timer: u8,
}

impl Iterator for Lanternfish {
    type Item = Option<Self>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.timer {
            0 => {
                self.timer = 6;
                Some(Some(Self::new()))
            }

            _ => {
                self.timer -= 1;
                Some(None)
            }
        }
    }
}

impl Lanternfish {
    fn new() -> Self {
        Self { timer: 8 }
    }

    fn with_timer(timer: u8) -> Self {
        Self { timer }
    }
}

struct LanternfishSchool {
    fishes: Vec<Lanternfish>,
}

impl LanternfishSchool {
    fn new(timers: &[u8]) -> Self {
        Self {
            fishes: timers
                .iter()
                .map(|timer| Lanternfish::with_timer(*timer))
                .collect(),
        }
    }

    fn advance_all(&mut self) {
        let new_fishes = self
            .fishes
            .iter_mut()
            .map(|fish| fish.next().unwrap())
            .filter_map(|fish| fish)
            .collect::<Vec<_>>();

        for fish in new_fishes {
            self.fishes.push(fish);
        }
    }
}

impl Iterator for LanternfishSchool {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_all();
        Some(self.fishes.len())
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
    type Output = u32;

    fn input() -> &'static str {
        include_str!("./inputs/6.txt")
    }

    fn solve_first(input: &str) -> Self::Output {
        let input = Self::parse_input(input);
        let mut school = LanternfishSchool::new(&input);

        school.nth(80 - 1).unwrap() as u32
    }

    fn solve_second(input: &str) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (351188, 0)
    }
}
