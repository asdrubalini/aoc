use itertools::Itertools;

use crate::aoc::Solution;

#[derive(Debug, Clone, Copy)]
pub struct Box {
    l: u16,
    w: u16,
    h: u16,
}

impl From<(u16, u16, u16)> for Box {
    fn from(tuple: (u16, u16, u16)) -> Self {
        Self {
            l: tuple.0,
            w: tuple.1,
            h: tuple.2,
        }
    }
}

impl Box {
    fn required_paper_area(&self) -> u32 {
        let sides = [self.l * self.w, self.w * self.h, self.h * self.l];
        let smallest_side = *sides.iter().min().unwrap();

        sides.iter().sum::<u16>() as u32 * 2 + smallest_side as u32
    }

    fn required_ribbon_length(&self) -> u32 {
        let perimeters = [self.l + self.w, self.w + self.h, self.h + self.l]
            .into_iter()
            .map(|p| p * 2)
            .collect_vec();

        let smallest_perimeter = *perimeters.iter().min().unwrap() as u32;
        let bow = (self.l * self.w * self.h) as u32;

        smallest_perimeter + bow
    }
}

pub struct Two;

impl Two {
    fn parse_line(line: &str) -> Box {
        let dimensions = line.split('x').collect_vec();

        dimensions
            .into_iter()
            .map(|el| el.parse().unwrap())
            .collect_tuple::<(u16, u16, u16)>()
            .unwrap()
            .into()
    }
}

impl Solution for Two {
    type Output = u32;
    type Parsed = Vec<Box>;

    fn input() -> &'static str {
        include_str!("../inputs/2.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Self::parse_line).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        parsed.iter().map(|b| b.required_paper_area()).sum()
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        parsed.iter().map(|b| b.required_ribbon_length()).sum()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1598415, 3812909)
    }
}
