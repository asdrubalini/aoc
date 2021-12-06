use advent_of_code_2021::aoc::{DayFour, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = DayFour::input();

    c.bench_function("DayFour - first part", |b| {
        b.iter(|| {
            DayFour::solve_first(input);
        })
    });

    c.bench_function("DayFour - second part", |b| {
        b.iter(|| {
            DayFour::solve_second(input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
