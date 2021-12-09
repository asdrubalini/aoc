#![allow(dead_code, unused_macros, unused_variables)]

pub mod aoc;

/// Generate tests for day n
macro_rules! test_day {
    ($struct_name:ident) => {
        paste! {
            #[test]
            fn [<test_ $struct_name:snake>]() {
                $struct_name::assert_solutions();
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::aoc::*;
    use paste::paste;

    test_day!(DayOne);
    test_day!(DayTwo);
    test_day!(DayThree);
    test_day!(DayFour);
    test_day!(DayFive);
    test_day!(DaySix);
    test_day!(DaySeven);
    test_day!(DayEight);
    test_day!(DayNine);
}
