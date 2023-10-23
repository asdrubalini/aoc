use itertools::Itertools;

use crate::aoc::Solution;

#[derive(Debug)]
pub struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl From<&str> for Ingredient {
    fn from(line: &str) -> Self {
        // Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5
        let tokens = line.split(' ').collect_vec();

        let name = tokens.first().unwrap().replace(":", "");

        let capacity = tokens.get(2).unwrap().replace(",", "").parse().unwrap();
        let durability = tokens.get(4).unwrap().replace(",", "").parse().unwrap();
        let flavor = tokens.get(6).unwrap().replace(",", "").parse().unwrap();
        let texture = tokens.get(8).unwrap().replace(",", "").parse().unwrap();
        let calories = tokens.get(10).unwrap().replace(",", "").parse().unwrap();

        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[derive(Debug)]
pub struct Ingredients {
    inner: Vec<Ingredient>,
}

impl FromIterator<Ingredient> for Ingredients {
    fn from_iter<T: IntoIterator<Item = Ingredient>>(iter: T) -> Self {
        Ingredients {
            inner: iter.into_iter().collect_vec(),
        }
    }
}

impl Ingredients {
    fn compute_total_score(&self, teaspoons: &[i32]) -> u32 {
        let capacity: i32 = self
            .inner
            .iter()
            .enumerate()
            .map(|(i, ingredient)| ingredient.capacity * teaspoons.get(i).unwrap())
            .sum();

        if capacity <= 0 {
            return 0;
        }

        let durability: i32 = self
            .inner
            .iter()
            .enumerate()
            .map(|(i, ingredient)| ingredient.durability * teaspoons.get(i).unwrap())
            .sum();

        if durability <= 0 {
            return 0;
        }

        let flavor: i32 = self
            .inner
            .iter()
            .enumerate()
            .map(|(i, ingredient)| ingredient.flavor * teaspoons.get(i).unwrap())
            .sum();

        if flavor <= 0 {
            return 0;
        }

        let texture: i32 = self
            .inner
            .iter()
            .enumerate()
            .map(|(i, ingredient)| ingredient.texture * teaspoons.get(i).unwrap())
            .sum();

        if texture <= 0 {
            return 0;
        }

        capacity as u32 * durability as u32 * flavor as u32 * texture as u32
    }
}

fn increment(teaspoons: &mut Vec<i32>) {
    let mut idx = teaspoons.len() - 1;

    loop {
        let teaspoon = teaspoons.get_mut(idx).unwrap();
        *teaspoon = if *teaspoon == 100 { 0 } else { *teaspoon + 1 };

        if *teaspoon != 0 {
            break;
        } else {
            if idx == 0 {
                return;
            }

            idx -= 1;
        }
    }
}

pub struct Fourteen;

impl Solution for Fourteen {
    type Output = u32;
    type Parsed = Ingredients;

    fn input() -> &'static str {
        include_str!("../inputs/14.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Ingredient::from).collect()
    }

    fn solve_first(ingredients: &Self::Parsed) -> Self::Output {
        let mut teaspoons = vec![0; ingredients.inner.len()];

        let mut max = 0;
        let steps = 100usize.pow(teaspoons.len() as u32);

        for _ in 0..steps {
            if teaspoons.iter().copied().sum::<i32>() != 100 {
                increment(&mut teaspoons);
                continue;
            }

            let score = ingredients.compute_total_score(&teaspoons);

            if score > max {
                max = score;
            }

            increment(&mut teaspoons);
        }

        max
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (18965440, 0)
    }
}
