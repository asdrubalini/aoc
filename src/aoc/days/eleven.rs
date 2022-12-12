use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Eleven;

#[derive(Debug, Clone)]
pub enum Operation {
    Sum(u8),
    Product(u8),
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
    fn apply(&self, mut worry_level: WorryLevel) -> WorryLevel {
        match self {
            Operation::Sum(a) => worry_level.sum(*a as u32),
            Operation::Product(a) => worry_level.multiply(*a as u32),
            Operation::Square => worry_level.square(),
        }

        worry_level
    }
}

#[derive(Debug, Clone)]
pub struct WorryLevel {
    modulos: HashMap<u8, u32>,
}

impl WorryLevel {
    fn sum(&mut self, other: u32) {
        for (modulo, value) in self.modulos.iter_mut() {
            *value += other;
            *value %= *modulo as u32;
        }
    }

    fn multiply(&mut self, other: u32) {
        for (modulo, value) in self.modulos.iter_mut() {
            *value *= other;
            *value %= *modulo as u32;
        }
    }

    fn square(&mut self) {
        for (modulo, value) in self.modulos.iter_mut() {
            *value *= *value;
            *value %= *modulo as u32;
        }
    }

    fn divide_floor(&mut self, other: u32) {
        for (modulo, value) in self.modulos.iter_mut() {
            *value += other;
            *value %= *modulo as u32;
        }
    }

    fn is_divisible_by(&self, divisor: u8) -> bool {
        *self.modulos.get(&divisor).unwrap() == 0
    }
}

impl From<u32> for WorryLevel {
    fn from(worry_level: u32) -> Self {
        let modulos = [2, 3, 5, 7, 11, 13, 17, 19, 23]
            .into_iter()
            .map(|modulo| (modulo as u8, worry_level as u32 % modulo))
            .collect();

        WorryLevel { modulos }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items_worry_level: VecDeque<WorryLevel>,
    operation: Operation,
    test_divisible_by: u8,
    condition_true_monkey: usize,
    condition_false_monkey: usize,
    inspection_count: u32,
}

impl From<&str> for Monkey {
    fn from(monkey: &str) -> Self {
        let mut lines = monkey.lines().map(|line| line.trim());

        // skip id
        lines.next();

        let items_worry_level = {
            let line = lines.next().unwrap().replace("Starting items: ", "");
            line.split(", ")
                .map(|item| item.parse::<u32>().unwrap())
                .map(WorryLevel::from)
                .collect_vec()
        };

        let operation = {
            let line = lines.next().unwrap().replace("Operation: new = old ", "");
            Operation::from(line.as_str())
        };

        let test_divisible_by = {
            let divisible_by = lines.next().unwrap().replace("Test: divisible by ", "");
            divisible_by.parse::<u8>().unwrap()
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
    type Output = u128;
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

                while let Some(worry_level) = monkey.items_worry_level.pop_front() {
                    let mut worry_level = monkey.operation.apply(worry_level);
                    worry_level.divide_floor(3);

                    let monkey_id = if worry_level.is_divisible_by(monkey.test_divisible_by) {
                        monkey.condition_true_monkey
                    } else {
                        monkey.condition_false_monkey
                    };

                    let destination_monkey = monkeys.get_mut(monkey_id).unwrap();
                    destination_monkey
                        .items_worry_level
                        .push_back(worry_level.into());
                }
            }
        }

        monkeys
            .into_iter()
            .map(|m| m.inspection_count)
            .sorted()
            .rev()
            .take(2)
            .product::<u32>() as u128
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        let mut monkeys = parsed.to_owned();

        for _ in 0..10_000 {
            for monkey_id in 0..monkeys.len() {
                let mut monkey = {
                    let monkey_mut = monkeys.get_mut(monkey_id).unwrap();
                    monkey_mut.inspection_count += monkey_mut.items_worry_level.len() as u32;
                    let monkey = monkey_mut.to_owned();

                    // take out the items
                    monkey_mut.items_worry_level = VecDeque::new();

                    monkey
                };

                while let Some(worry_level) = monkey.items_worry_level.pop_front() {
                    let worry_level = monkey.operation.apply(worry_level);

                    let monkey_id = if worry_level.is_divisible_by(monkey.test_divisible_by) {
                        monkey.condition_true_monkey
                    } else {
                        monkey.condition_false_monkey
                    };

                    let destination_monkey = monkeys.get_mut(monkey_id).unwrap();
                    destination_monkey.items_worry_level.push_back(worry_level);
                }
            }
        }

        println!(
            "{:?}",
            monkeys.iter().map(|m| m.inspection_count).collect_vec()
        );

        monkeys
            .into_iter()
            .map(|m| m.inspection_count)
            .sorted()
            .rev()
            .take(2)
            .map(|n| n as u128)
            .product::<u128>()
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (62491, 0)
    }
}
