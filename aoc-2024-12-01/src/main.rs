//! 2024 Day 1: Historian Hysteria
//! Comparing three sorting approaches: C qsort, Rust sort, C++ std::sort

use std::ffi::c_void;
use std::os::raw::c_int;

// ============================================================================
// C++ std::sort via autocxx
// ============================================================================

use autocxx::prelude::*;

include_cpp! {
    #include "cpp_sort.h"
    safety!(unsafe)

    generate!("cpp_sorting::sort_i32_array")
}

// FFI declaration for libc's qsort
unsafe extern "C" {
    // https://www.tutorialspoint.com/c_standard_library/c_function_qsort.htm
    fn qsort(
        // the pointer to the first element of the array to be sorted
        base: *mut c_void,
        // number of elements in the array
        num: usize,
        // size of each element in the array
        size: usize,
        // a function pointer to a 2-element comparison function
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int
    );
}

// Comparison function for qsort (ascending order)
// Must return: negative if a < b, zero if a == b, positive if a > b
unsafe extern "C" fn compare_i32(a: *const c_void, b: *const c_void) -> c_int {
    // SAFETY: qsort guarantees valid pointers to i32 elements
    unsafe {
        let a = *(a as *const i32);
        let b = *(b as *const i32);
        a - b  // Simple subtraction works for i32 comparisons vs `a.cmp(&b) as c_int`
    }
}

// Wrapper that calls C's qsort on a Rust Vec<i32>
// SAFETY: Vec's memory layout is compatible with C arrays (contiguous, aligned)
// qsort won't resize/reallocate, just reorders elements in place
fn c_qsort(vec: &mut Vec<i32>) {
    unsafe {
        qsort(
            // pointer to the first element in the array
            vec.as_mut_ptr() as *mut c_void,
            // number of elements in the array
            vec.len(),
            // size of each element in the array
            std::mem::size_of::<i32>(),
            // comparison function
            compare_i32
        );
    }
}

// Wrapper that calls C++ std::sort via autocxx
fn cpp_sort(vec: &mut Vec<i32>) {
    // Sort in-place: pass raw pointer + length directly to C++
    // SAFETY: Vec's memory is contiguous and aligned, std::sort won't resize
    unsafe {
        ffi::cpp_sorting::sort_i32_array(vec.as_mut_ptr(), vec.len());
    }
}

/// transpose 2 colums of numbers
fn unzip(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let nums = line
                // from columns
                .split_whitespace()
                // parse number
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (nums[0], nums[1])
        })
        .unzip()
}

// Process using C qsort
fn process_c_qsort(input: &str) -> Result<i32, String> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    c_qsort(&mut left);
    c_qsort(&mut right);

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l-r).abs())
        .sum::<i32>()
    )
}

// Process using Rust's built-in sort
fn process_rust_sort(input: &str) -> Result<i32, String> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l-r).abs())
        .sum::<i32>()
    )
}

// Process using C++ std::sort via autocxx
fn process_cpp_sort(input: &str) -> Result<i32, String> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    cpp_sort(&mut left);
    cpp_sort(&mut right);

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l-r).abs())
        .sum::<i32>()
    )
}

fn main() {
    println!("2024 Day 1: Comparing Rust, C, and C++ sorting approaches\n");

    let example = "3   4
4   3
2   5
1   3
3   9
3   3";

    println!("Testing all three approaches:");
    println!("  C qsort:      {}", process_c_qsort(example).expect("11"));
    println!("  Rust sort:    {}", process_rust_sort(example).expect("11"));
    println!("  C++ std::sort: {}", process_cpp_sort(example).expect("11"));

    println!("\nAll approaches should give the same result: 11");

    // Correctness tests
    assert_eq!(process_c_qsort("3 7").unwrap(), 4);
    assert_eq!(process_rust_sort("3 7").unwrap(), 4);
    assert_eq!(process_cpp_sort("3 7").unwrap(), 4);

    println!("\n✓ All tests passed!");
}