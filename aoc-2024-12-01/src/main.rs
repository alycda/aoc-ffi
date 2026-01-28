//! 2024 Day 1: Historian Hysteria - CLI Entry Point

use aoc_2024_12_01::{SAMPLE_INPUT, process_c_qsort, process_rust_sort, process_rust_sort_stable};

fn main() {
    println!("2024 Day 1: Comparing C qsort vs Rust sort\n");

    println!("Testing all three approaches:");
    println!("  C qsort:             {}", process_c_qsort(SAMPLE_INPUT).expect("11"));
    println!("  Rust sort (unstable): {}", process_rust_sort(SAMPLE_INPUT).expect("11"));
    println!("  Rust sort (stable):   {}", process_rust_sort_stable(SAMPLE_INPUT).expect("11"));

    // Run some quick validation
    let c = process_c_qsort("3 7").expect("4");
    let unstable = process_rust_sort("3 7").expect("4");
    let stable = process_rust_sort_stable("3 7").expect("4");
    assert_eq!(c, unstable);
    assert_eq!(c, stable);

    println!("\n✓ All tests passed!");
    println!("\nRun `cargo bench` to see performance comparison");
}
