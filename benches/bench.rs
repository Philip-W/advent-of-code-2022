use advent_of_code_2022::days::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_days(c: &mut Criterion) {
    c.bench_function("day 01", |b| {
        b.iter(|| day01::run(black_box(include_str!("../input/real/01.txt"))))
    });
    c.bench_function("day 02", |b| {
        b.iter(|| day02::run(black_box(include_str!("../input/real/02.txt"))))
    });
    c.bench_function("day 03", |b| {
        b.iter(|| day03::run(black_box(include_str!("../input/real/03.txt"))))
    });
    c.bench_function("day 04", |b| {
        b.iter(|| day04::run(black_box(include_str!("../input/real/04.txt"))))
    });
    c.bench_function("day 05", |b| {
        b.iter(|| day05::run(black_box(include_str!("../input/real/05.txt"))))
    });
    c.bench_function("day 05", |b| {
        b.iter(|| day06::run(black_box(include_str!("../input/real/06.txt"))))
    });
}

criterion_group!(benches, bench_days);
criterion_main!(benches);
