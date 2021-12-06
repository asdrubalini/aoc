use advent_of_code_2021::aoc::{DayTwo, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = DayTwo::input();

    c.bench_function("DayTwo - first part", |b| {
        b.iter(|| {
            DayTwo::solve_first(input);
        })
    });

    c.bench_function("DayTwo - second part", |b| {
        b.iter(|| {
            DayTwo::solve_second(input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
