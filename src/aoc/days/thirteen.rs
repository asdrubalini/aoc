use itertools::Itertools;

use crate::aoc::Solution;

#[derive(Debug, Clone, Copy)]
pub enum ReindeerStatus {
    Flying,
    Resting,
}

#[derive(Debug, Clone, Copy)]
pub struct Reindeer {
    speed: u16,
    fly_time: u16,
    rest_time: u16,

    status: ReindeerStatus,

    flying_for: u16,
    resting_for: u16,
    space_travelled: u32,

    points: u32,
}

impl From<&str> for Reindeer {
    fn from(line: &str) -> Self {
        let tokens: Vec<&str> = line.split(' ').collect_vec();

        let speed = tokens.get(3).unwrap().parse().unwrap();
        let fly_time = tokens.get(6).unwrap().parse().unwrap();
        let rest_time = tokens.get(13).unwrap().parse().unwrap();

        Reindeer {
            speed,
            fly_time,
            rest_time,

            status: ReindeerStatus::Flying,

            flying_for: 0,
            resting_for: 0,
            space_travelled: 0,

            points: 0,
        }
    }
}

impl Reindeer {
    fn tick(&mut self) {
        match self.status {
            ReindeerStatus::Flying => {
                if self.flying_for == self.fly_time {
                    self.flying_for = 0;
                    self.status = ReindeerStatus::Resting;
                    self.resting_for = 1;
                } else {
                    self.flying_for += 1;
                    self.space_travelled += self.speed as u32;
                }
            }

            ReindeerStatus::Resting => {
                if self.resting_for == self.rest_time {
                    self.resting_for = 0;
                    self.status = ReindeerStatus::Flying;

                    self.flying_for = 1;
                    self.space_travelled += self.speed as u32;
                } else {
                    self.resting_for += 1;
                }
            }
        }
    }

    fn make_tick_winner(&mut self) {
        self.points += 1;
    }
}

#[derive(Clone, Debug)]
pub struct Reindeers {
    inner: Vec<Reindeer>,
}

impl FromIterator<Reindeer> for Reindeers {
    fn from_iter<T: IntoIterator<Item = Reindeer>>(iter: T) -> Self {
        Reindeers {
            inner: iter.into_iter().collect_vec(),
        }
    }
}

impl Reindeers {
    fn tick(&mut self) {
        for reindeer in self.inner.iter_mut() {
            reindeer.tick();
        }

        for reindeer in self.furthest_reindeers() {
            reindeer.make_tick_winner();
        }
    }

    fn furthest_reindeers(&mut self) -> Vec<&mut Reindeer> {
        self.inner
            .sort_by(|a, b| a.space_travelled.cmp(&b.space_travelled));

        let first = *self.inner.last().unwrap();

        self.inner
            .iter_mut()
            .filter(|reindeer| reindeer.space_travelled == first.space_travelled)
            .collect_vec()
    }

    fn winner(&self) -> Reindeer {
        let mut sorted = self.inner.clone();
        sorted.sort_by(|a, b| a.points.cmp(&b.points));

        *sorted.last().unwrap()
    }
}

pub struct Thirteen;

impl Solution for Thirteen {
    type Output = u32;
    type Parsed = Reindeers;

    fn input() -> &'static str {
        include_str!("../inputs/13.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Reindeer::from).collect()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut reindeers = parsed.to_owned();

        for _ in 0..2503 {
            reindeers.tick();
        }

        reindeers
            .furthest_reindeers()
            .first()
            .unwrap()
            .space_travelled
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut reindeers = parsed.to_owned();

        for _ in 0..2503 {
            reindeers.tick();
        }

        reindeers.winner().points
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (2655, 1059)
    }
}
