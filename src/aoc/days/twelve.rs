

use serde_json::Value;

use crate::aoc::Solution;

fn sum_all_v1(value: &Value) -> i64 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => panic!("Bool not expected"),
        Value::Number(num) => num.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(values) => values.iter().map(sum_all_v1).sum(),
        Value::Object(map) => map.values().map(sum_all_v1).sum(),
    }
}

fn sum_all_v2(value: &Value) -> i64 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => panic!("Bool not expected"),
        Value::Number(num) => num.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(values) => values.iter().map(sum_all_v2).sum(),
        Value::Object(map) => {
            // Ignore if red is in the values

            if map
                .values()
                .filter(|v| match v {
                    Value::String(v) => v == "red",
                    _ => false,
                })
                .count()
                >= 1
            {
                return 0;
            }

            map.values().map(sum_all_v2).sum()
        }
    }
}

pub struct Twelve;

impl Solution for Twelve {
    type Output = i64;
    type Parsed = Value;

    fn input() -> &'static str {
        include_str!("../inputs/12.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        serde_json::from_str(input).unwrap()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        sum_all_v1(parsed)
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        sum_all_v2(parsed)
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (119433, 68466)
    }
}
