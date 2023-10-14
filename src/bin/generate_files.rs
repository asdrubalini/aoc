use std::{env::current_dir, fs::File, io::Write};

use convert_case::Casing;

fn main() {
    let s = "use crate::aoc::Solution;

pub struct DAY_NAME;

impl Solution for DAY_NAME {
    type Output = u32;
    type Parsed = Vec<u32>;

    fn input() -> &'static str {
        \"\"
    }

    fn parse_input(_input: &'static str) -> Self::Parsed {
        vec![]
    }

    fn solve_first(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}";

    let days = vec![
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
        "Twenty",
        "TwentyOne",
        "TwentyTwo",
        "TwentyThree",
        "TwentyFour",
        "TwentyFive",
    ];

    let mut mod_content = String::new();

    for day in days {
        let content = s.replace("DAY_NAME", day);
        let snake_name = day.to_case(convert_case::Case::Snake);

        let mut path = current_dir().unwrap();
        path.push(format!("src/aoc/days/{snake_name}.rs"));

        mod_content.push_str(&format!(
            "mod {snake_name};\npub use {snake_name}::{day};\n"
        ));

        let mut f = File::create(path).unwrap();
        f.write_all(content.as_bytes()).unwrap();

        // println!("{day}::benchmark()+")
        // println!("test_day!({day});");
    }

    let mut path = current_dir().unwrap();
    path.push("src/aoc/days/mod.rs");

    let mut f = File::create(path).unwrap();
    f.write_all(mod_content.as_bytes()).unwrap();
}
