use advent_of_code_2021::aoc::{DayThree, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = DayThree::input();

    c.bench_function("DayThree - first part", |b| {
        b.iter(|| {
            DayThree::solve_first(input);
        })
    });

    c.bench_function("DayThree - second part", |b| {
        b.iter(|| {
            DayThree::solve_second(input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
