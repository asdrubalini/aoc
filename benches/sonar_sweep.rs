use advent_of_code_2021::aoc::{DayOne, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = DayOne::input();

    c.bench_function("DayOne - first part", |b| {
        b.iter(|| {
            DayOne::solve_first(input);
        })
    });

    c.bench_function("DayOne - second part", |b| {
        b.iter(|| {
            DayOne::solve_second(input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
