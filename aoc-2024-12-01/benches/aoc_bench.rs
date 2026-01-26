use aoc_2024_12_01::{SAMPLE_INPUT, process_c_qsort, process_rust_sort, process_part_2, process_part_2_hashmap};
#[cfg(feature = "glib")]
use aoc_2024_12_01::process_part_2_glib;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_process_c_qsort(c: &mut Criterion) {
    c.bench_function("process c qsort", |b| {
        b.iter(|| process_c_qsort(black_box(SAMPLE_INPUT)))
    });
}

fn benchmark_process_rust_sort(c: &mut Criterion) {
    c.bench_function("process rust sort", |b| {
        b.iter(|| process_rust_sort(black_box(SAMPLE_INPUT)))
    });
}

fn benchmark_process_part_2(c: &mut Criterion) {
    c.bench_function("part 2 naive", |b| {
        b.iter(|| process_part_2(black_box(SAMPLE_INPUT)))
    });
}

fn benchmark_process_part_2_hashmap(c: &mut Criterion) {
    c.bench_function("part 2 hashmap", |b| {
        b.iter(|| process_part_2_hashmap(black_box(SAMPLE_INPUT)))
    });
}

#[cfg(feature = "glib")]
fn benchmark_process_part_2_glib(c: &mut Criterion) {
    c.bench_function("part 2 glib", |b| {
        b.iter(|| process_part_2_glib(black_box(SAMPLE_INPUT)))
    });
}

#[cfg(feature = "glib")]
criterion_group!(benches, benchmark_process_c_qsort, benchmark_process_rust_sort, benchmark_process_part_2, benchmark_process_part_2_hashmap, benchmark_process_part_2_glib);
#[cfg(not(feature = "glib"))]
criterion_group!(benches, benchmark_process_c_qsort, benchmark_process_rust_sort, benchmark_process_part_2, benchmark_process_part_2_hashmap);
criterion_main!(benches);
