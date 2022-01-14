use std::time::Duration;

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
    group.bench_function("Day 11; Pt.1", |b| b.iter(aoc2021::day11::first));
    group.bench_function("Day 11; Pt.2", |b| b.iter(aoc2021::day11::second));
    group.bench_function("Day 12; Pt.1", |b| b.iter(aoc2021::day12::first));
    group.bench_function("Day 12; Pt.2", |b| b.iter(aoc2021::day12::second));
    group.bench_function("Day 13; Pt.1", |b| b.iter(aoc2021::day13::first));
    group.bench_function("Day 13; Pt.2", |b| b.iter(|| aoc2021::day13::second(false)));
    group.bench_function("Day 14; Pt.1", |b| b.iter(aoc2021::day14::first));
    group.bench_function("Day 14; Pt.2", |b| b.iter(aoc2021::day14::second));
    group.bench_function("Day 15; Pt.1", |b| b.iter(aoc2021::day15::first));
    group.bench_function("Day 15; Pt.2", |b| b.iter(aoc2021::day15::second));
    group.bench_function("Day 16; Pt.1", |b| b.iter(aoc2021::day16::first));
    group.bench_function("Day 16; Pt.2", |b| b.iter(aoc2021::day16::second));
    group.bench_function("Day 17; Pt.1", |b| b.iter(aoc2021::day17::first));
    group.bench_function("Day 17; Pt.2", |b| b.iter(aoc2021::day17::second));
    group.finish();
}

fn all(c: &mut Criterion) {
    let mut group = c.benchmark_group("Advent of Code 2021");
    group.measurement_time(Duration::from_secs(60));
    group.warm_up_time(Duration::from_secs(5));
    group.sample_size(500);
    group.bench_function("All", |b| {
        b.iter(|| {
            let _ = aoc2021::day1::first();
            let _ = aoc2021::day1::second();
            let _ = aoc2021::day2::first();
            let _ = aoc2021::day2::second();
            let _ = aoc2021::day3::first();
            let _ = aoc2021::day3::second();
            let _ = aoc2021::day4::first();
            let _ = aoc2021::day4::second();
            let _ = aoc2021::day5::first();
            let _ = aoc2021::day5::second();
            let _ = aoc2021::day6::first();
            let _ = aoc2021::day6::second();
            let _ = aoc2021::day7::first();
            let _ = aoc2021::day7::second();
            let _ = aoc2021::day8::first();
            let _ = aoc2021::day8::second();
            let _ = aoc2021::day9::first();
            let _ = aoc2021::day9::second();
            let _ = aoc2021::day10::first();
            let _ = aoc2021::day10::second();
            let _ = aoc2021::day11::first();
            let _ = aoc2021::day11::second();
            let _ = aoc2021::day12::first();
            let _ = aoc2021::day12::second();
            let _ = aoc2021::day13::first();
            let _ = aoc2021::day13::second(false);
            let _ = aoc2021::day14::first();
            let _ = aoc2021::day14::second();
            let _ = aoc2021::day15::first();
            let _ = aoc2021::day15::second();
            let _ = aoc2021::day16::first();
            let _ = aoc2021::day16::second();
            let _ = aoc2021::day17::first();
            let _ = aoc2021::day17::second();
        });
    });
    group.finish();
}

criterion_group!(all_benchmarks, all);
criterion_group!(individual_bench, benchmarks);
criterion_main!(all_benchmarks, individual_bench);
