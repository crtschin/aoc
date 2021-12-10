use criterion::{criterion_group, criterion_main, Criterion};

use aoc::aoc2021;

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Advent of Code 2021");
    group.bench_function("Day 1; Pt.1", |b| b.iter(aoc2021::day1::first));
    group.bench_function("Day 1; Pt.2", |b| b.iter(aoc2021::day1::second));
    group.bench_function("Day 2; Pt.1", |b| b.iter(aoc2021::day2::first));
    group.bench_function("Day 2; Pt.2", |b| b.iter(aoc2021::day2::second));
    group.bench_function("Day 3; Pt.1", |b| b.iter(aoc2021::day3::first));
    group.bench_function("Day 3; Pt.2", |b| b.iter(aoc2021::day3::second));
    group.bench_function("Day 4; Pt.1", |b| b.iter(aoc2021::day4::first));
    group.bench_function("Day 4; Pt.2", |b| b.iter(aoc2021::day4::second));
    group.bench_function("Day 5; Pt.1", |b| b.iter(aoc2021::day5::first));
    group.bench_function("Day 5; Pt.2", |b| b.iter(aoc2021::day5::second));
    group.bench_function("Day 6; Pt.1", |b| b.iter(aoc2021::day6::first));
    group.bench_function("Day 6; Pt.2", |b| b.iter(aoc2021::day6::second));
    group.bench_function("Day 7; Pt.1", |b| b.iter(aoc2021::day7::first));
    group.bench_function("Day 7; Pt.2", |b| b.iter(aoc2021::day7::second));
    group.bench_function("Day 8; Pt.1", |b| b.iter(aoc2021::day8::first));
    group.bench_function("Day 8; Pt.2", |b| b.iter(aoc2021::day8::second));
    group.bench_function("Day 9; Pt.1", |b| b.iter(aoc2021::day9::first));
    group.bench_function("Day 9; Pt.2", |b| b.iter(aoc2021::day9::second));
    group.bench_function("Day 10; Pt.1", |b| b.iter(aoc2021::day10::first));
    group.bench_function("Day 10; Pt.2", |b| b.iter(aoc2021::day10::second));
    group.finish();
}

criterion_group!(bench, benchmarks);
criterion_main!(bench);
