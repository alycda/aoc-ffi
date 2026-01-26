use aoc_ffi_day01::{process_c_qsort, process_rust_sort, process_part_2, process_part_2_hashmap, process_part_2_ahash, process_part_2_uthash};
#[cfg(feature = "glib")]
use aoc_ffi_day01::process_part_2_glib;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const REAL_INPUT: &'static str = include_str!("./REAL_INPUT.txt");

fn benchmark_process_c_qsort(c: &mut Criterion) {
    c.bench_function("process c qsort", |b| {
        b.iter(|| process_c_qsort(black_box(REAL_INPUT)))
    });
}

fn benchmark_process_rust_sort(c: &mut Criterion) {
    c.bench_function("process rust sort", |b| {
        b.iter(|| process_rust_sort(black_box(REAL_INPUT)))
    });
}

fn benchmark_process_part_2(c: &mut Criterion) {
    c.bench_function("part 2 naive", |b| {
        b.iter(|| process_part_2(black_box(REAL_INPUT)))
    });
}

fn benchmark_process_part_2_hashmap(c: &mut Criterion) {
    c.bench_function("part 2 hashmap", |b| {
        b.iter(|| process_part_2_hashmap(black_box(REAL_INPUT)))
    });
}

#[cfg(feature = "glib")]
fn benchmark_process_part_2_glib(c: &mut Criterion) {
    c.bench_function("part 2 glib", |b| {
        b.iter(|| process_part_2_glib(black_box(REAL_INPUT)))
    });
}

fn benchmark_process_part_2_ahash(c: &mut Criterion) {
    c.bench_function("part 2 ahash", |b| {
        b.iter(|| process_part_2_ahash(black_box(REAL_INPUT)))
    });
}

fn benchmark_process_part_2_uthash(c: &mut Criterion) {
    c.bench_function("part 2 uthash", |b| {
        b.iter(|| process_part_2_uthash(black_box(REAL_INPUT)))
    });
}

#[cfg(feature = "glib")]
criterion_group!(benches, benchmark_process_c_qsort, benchmark_process_rust_sort, benchmark_process_part_2, benchmark_process_part_2_hashmap, benchmark_process_part_2_ahash, benchmark_process_part_2_glib, benchmark_process_part_2_uthash);
#[cfg(not(feature = "glib"))]
criterion_group!(benches, benchmark_process_c_qsort, benchmark_process_rust_sort, benchmark_process_part_2, benchmark_process_part_2_hashmap, benchmark_process_part_2_ahash, benchmark_process_part_2_uthash);
criterion_main!(benches);
