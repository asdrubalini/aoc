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
            Operation::Sum(a) => {
                worry_level.worry_level += *a as u64;
            }
            Operation::Product(a) => {
                worry_level.worry_level *= *a as u64;
            }
            Operation::Square => {
                worry_level.worry_level *= worry_level.worry_level;
            }
        }

        worry_level
    }

    fn apply_soft(&self, mut worry_level: WorryLevel) -> WorryLevel {
        match self {
            Operation::Sum(a) => {
                let sum = worry_level.worry_level + *a as u64;
                worry_level.worry_level = sum;
                worry_level.divisible_by = WorryLevel::find_dividends(sum);
            }
            Operation::Product(a) => match *a {
                3 => {
                    worry_level.set_divisible(3);
                }
                19 => {
                    worry_level.set_divisible(19);
                }
                _ => panic!("wtf"),
            },
            Operation::Square => {
                //*worry_level.divisible_by.get_mut(&2).unwrap() = true;
            }
        }

        worry_level
    }
}

#[derive(Debug, Clone)]
pub struct WorryLevel {
    worry_level: u64,
    divisible_by: HashMap<u8, bool>,
}

impl WorryLevel {
    fn is_divisible_by(&self, dividend: u8) -> bool {
        *self.divisible_by.get(&dividend).unwrap()
    }

    fn find_dividends(worry_level: u64) -> HashMap<u8, bool> {
        [2, 3, 5, 7, 11, 13, 17, 19, 23]
            .map(|dividend| (dividend as u8, worry_level % dividend == 0))
            .into()
    }

    fn set_divisible(&mut self, dividend: u8) {
        let ciao = self.divisible_by.get_mut(&dividend).unwrap();
        *ciao = true;
    }
}

impl From<u64> for WorryLevel {
    fn from(worry_level: u64) -> Self {
        let divisible_by = Self::find_dividends(worry_level);

        WorryLevel {
            worry_level,
            divisible_by,
        }
    }
}

impl Into<u64> for WorryLevel {
    fn into(self) -> u64 {
        self.worry_level
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
                .map(|item| item.parse::<u64>().unwrap())
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
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
        //include_str!("../inputs/11.txt")
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
                    let worry_level = monkey.operation.apply(worry_level).worry_level as f64;
                    let worry_level = (worry_level / 3.0).floor();

                    let monkey_id = if worry_level % monkey.test_divisible_by as f64 == 0.0 {
                        monkey.condition_true_monkey
                    } else {
                        monkey.condition_false_monkey
                    };

                    let destination_monkey = monkeys.get_mut(monkey_id).unwrap();

                    destination_monkey
                        .items_worry_level
                        .push_back((worry_level as u64).into());
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
                    let worry_level = monkey.operation.apply_soft(worry_level);

                    let monkey_id = if worry_level.is_divisible_by(monkey.test_divisible_by as u8) {
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
