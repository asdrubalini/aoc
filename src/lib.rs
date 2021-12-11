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
}
