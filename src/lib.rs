#![allow(dead_code, unused_macros)]

mod aoc;

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
    use super::aoc::{solutions::*, Solution};
    use paste::paste;

    test_day!(DayOne);
    test_day!(DayTwo);
}
