use core::num;
use std::{collections::VecDeque, mem::take};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Eleven;

#[derive(Debug, Clone)]
pub enum Operation {
    Sum(u64),
    Product(u64),
    Square,
}

impl From<&str> for Operation {
    fn from(operation: &str) -> Self {
        let (operand, number) = operation.split_ascii_whitespace().collect_tuple().unwrap();

        if number == "old" {
            Operation::Square
        } else {
            let number = number.parse().unwrap();
            match operand {
                "+" => Operation::Sum(number),
                "*" => Operation::Product(number),
                _ => panic!("wtf"),
            }
        }
    }
}

impl Operation {
    fn apply(&self, a: u64) -> u64 {
        match self {
            Operation::Sum(b) => a + b,
            Operation::Product(b) => a * b,
            Operation::Square => a * a,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    items_worry_level: VecDeque<u64>,
    operation: Operation,
    test_divisible_by: u64,
    condition_true_monkey: usize,
    condition_false_monkey: usize,
    inspection_count: u32,
}

impl From<&str> for Monkey {
    fn from(monkey: &str) -> Self {
        let mut lines = monkey.lines().map(|line| line.trim());

        let id = {
            let line = lines
                .next()
                .unwrap()
                .replace("Monkey ", "")
                .replace(":", "");

            println!("{line}");
            line.parse::<usize>().unwrap()
        };

        let items_worry_level = {
            let line = lines.next().unwrap().replace("Starting items: ", "");
            line.split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect_vec()
        };

        let operation = {
            let line = lines.next().unwrap().replace("Operation: new = old ", "");
            Operation::from(line.as_str())
        };

        let test_divisible_by = {
            let divisible_by = lines.next().unwrap().replace("Test: divisible by ", "");
            divisible_by.parse::<u64>().unwrap()
        };

        let condition_true_monkey = {
            let monkey = lines
                .next()
                .unwrap()
                .replace("If true: throw to monkey ", "");

            monkey.parse::<usize>().unwrap()
        };

        let condition_false_monkey = {
            let monkey = lines
                .next()
                .unwrap()
                .replace("If false: throw to monkey ", "");

            monkey.parse::<usize>().unwrap()
        };

        Monkey {
            id,
            items_worry_level: VecDeque::from(items_worry_level),
            operation,
            test_divisible_by,
            condition_true_monkey,
            condition_false_monkey,

            inspection_count: 0,
        }
    }
}

impl Solution for Eleven {
    type Output = u32;
    type Parsed = Vec<Monkey>;

    fn input() -> &'static str {
        include_str!("../inputs/11.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input
            .split("\n\n")
            .into_iter()
            .map(Monkey::from)
            .collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let mut monkeys = parsed.to_owned();

        for _ in 0..20 {
            for monkey_id in 0..monkeys.len() {
                let mut monkey = {
                    let monkey_mut = monkeys.get_mut(monkey_id).unwrap();
                    monkey_mut.inspection_count += monkey_mut.items_worry_level.len() as u32;
                    let monkey = monkey_mut.to_owned();

                    // take out the items
                    monkey_mut.items_worry_level = VecDeque::new();

                    monkey
                };

                while let Some(item) = monkey.items_worry_level.pop_front() {
                    let worry_level = monkey.operation.apply(item) as f64;
                    let worry_level = (worry_level / 3.0).floor();

                    let other_monkey = monkeys
                        .get_mut(if worry_level % monkey.test_divisible_by as f64 == 0.0 {
                            monkey.condition_true_monkey
                        } else {
                            monkey.condition_false_monkey
                        })
                        .unwrap();

                    other_monkey.items_worry_level.push_back(worry_level as u64);
                }
            }
        }

        monkeys
            .into_iter()
            .map(|m| m.inspection_count)
            .sorted()
            .rev()
            .take(2)
            .product::<u32>()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (62491, 0)
    }
}
