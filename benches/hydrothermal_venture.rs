use advent_of_code_2021::aoc::{DayFive, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = DayFive::input();

    c.bench_function("DayFive - first part", |b| {
        b.iter(|| {
            DayFive::solve_first(input);
        })
    });

    c.bench_function("DayFive - second part", |b| {
        b.iter(|| {
            DayFive::solve_second(input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
