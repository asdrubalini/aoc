#![allow(dead_code, unused_macros)]

mod aoc;

/// Generate tests for day n
macro_rules! test_day {
    ($struct_name:ident) => {
        use paste::paste;

        paste! {
            #[test]
            fn [<test_ $struct_name:snake>]() {
                let input = $struct_name::input();
                let solution = $struct_name::solve(input);
                assert_eq!(solution, $struct_name::expected_solution());
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::aoc::{solutions::*, Solution};

    test_day!(DayOne);
}
