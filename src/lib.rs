#![allow(dead_code, unused_macros)]

mod aoc;

/// Generate tests for day n
macro_rules! test_day {
    ($struct_name:ident) => {
        use paste::paste;

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

    test_day!(DayOne);
}
