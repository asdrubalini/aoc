use aoc::{days::*, Solution};

mod aoc;

macro_rules! debug_day {
    ($struct_name:ident) => {
        let input = $struct_name::input();

        let parsed = $struct_name::parse_input(input);
        println!("Parsed: {:#?}", parsed);

        let solution_one = $struct_name::solve_first(&parsed);
        println!("First solution: {:?}", solution_one);
        let solution_two = $struct_name::solve_second(&parsed);
        println!("Second solution: {:?}", solution_two);
    };
}

fn main() {
    debug_day!(Fifteen);
}
