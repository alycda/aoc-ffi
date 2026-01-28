use aoc_2024_12_01::{SAMPLE_INPUT, process_c_qsort, process_rust_sort, process_rust_sort_stable};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_process_c_qsort(c: &mut Criterion) {
    c.bench_function("process c qsort", |b| {
        b.iter(|| process_c_qsort(black_box(SAMPLE_INPUT)))
    });
}

fn benchmark_process_rust_sort(c: &mut Criterion) {
    c.bench_function("process rust sort (unstable)", |b| {
        b.iter(|| process_rust_sort(black_box(SAMPLE_INPUT)))
    });
}

fn benchmark_process_rust_sort_stable(c: &mut Criterion) {
    c.bench_function("process rust sort (stable)", |b| {
        b.iter(|| process_rust_sort_stable(black_box(SAMPLE_INPUT)))
    });
}

criterion_group!(benches, benchmark_process_c_qsort, benchmark_process_rust_sort, benchmark_process_rust_sort_stable);
criterion_main!(benches);
