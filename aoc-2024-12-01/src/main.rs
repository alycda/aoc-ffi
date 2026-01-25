//! 2024 Day 1: Historian Hysteria - CLI Entry Point

use aoc_2024_12_01::{SAMPLE_INPUT, process_c_qsort, process_rust_sort};

fn main() {
    println!("2024 Day 1: Comparing C qsort vs Rust sort\n");

    println!("Testing both approaches:");
    println!("  C qsort:    {}", process_c_qsort(SAMPLE_INPUT).expect("11"));
    println!("  Rust sort:  {}", process_rust_sort(SAMPLE_INPUT).expect("11"));

    // Run some quick validation
    assert_eq!(process_c_qsort("3 7").expect("4"), process_rust_sort("3 7").expect("4"));

    println!("\n✓ All tests passed!");
    println!("\nRun `cargo bench` to see performance comparison");
}
