//! 2024 Day 1: Historian Hysteria - FFI Benchmarking Library
//!
//! NOTE: This example is intentionally contrived - Rust's built-in sort is both
//! safer and faster. The point is to demonstrate FFI mechanics (extern "C",
//! function pointers, unsafe) with a familiar algorithm before tackling more
//! complex FFI scenarios.

use std::ffi::c_void;
use std::os::raw::c_int;

// ============================================================================
// Zero-Cost Abstraction: Sorter Trait with Marker Types
// ============================================================================

/// Trait for different sorting strategies
///
/// This is a zero-cost abstraction because:
/// - Marker types are zero-sized (no runtime memory)
/// - Monomorphization creates specialized versions at compile time
/// - No vtables or dynamic dispatch - all resolved statically
pub trait Sorter {
    fn sort(vec: &mut Vec<i32>);
}

/// Marker type for native Rust sorting
pub struct NativeSort;

impl Sorter for NativeSort {
    fn sort(vec: &mut Vec<i32>) {
        vec.sort();
    }
}

/// Marker type for C qsort
pub struct CSort;

impl Sorter for CSort {
    fn sort(vec: &mut Vec<i32>) {
        c_qsort(vec);
    }
}

pub const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

// FFI declaration for libc's qsort
unsafe extern "C" {
    // Reference: https://www.tutorialspoint.com/c_standard_library/c_function_qsort.htm
    // Or see: man 3 qsort on any Unix system
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
    // NOTE: Rust 2024 edition requires unsafe {} blocks even inside unsafe fn
    // (In older editions, the inner unsafe {} would be redundant)
    let (a, b) = unsafe {
        let a = *(a as *const i32);
        let b = *(b as *const i32);
        (a, b)
    };

    // IMPORTANT: Don't use `a - b` here! It can overflow:
    // If a = i32::MAX and b = i32::MIN, subtraction panics (debug) or wraps (release)
    // The correct C idiom:
    if a < b {
        -1
    } else if a > b {
        1
    } else {
        0
    }

    // Alternative: use Rust's cmp (but shows less C idiom for teaching)
    // a.cmp(&b) as c_int
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

/// Transpose 2 columns of numbers
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

// ============================================================================
// Generic solve function - Zero-Cost Abstraction
// ============================================================================

/// Generic process function that works with any Sorter implementation
///
/// This function gets monomorphized at compile time for each concrete type S.
/// The compiler generates specialized versions: solve::<NativeSort>, solve::<CSort>, etc.
/// This means zero runtime overhead - it's as if you wrote separate functions by hand.
pub fn solve<S: Sorter>(input: &str) -> Result<i32, String> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    S::sort(&mut left);
    S::sort(&mut right);

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l-r).abs())
        .sum::<i32>()
    )
}

// ============================================================================
// Convenience wrapper functions - Public API
// ============================================================================

/// Process using C qsort
pub fn process_c_qsort(input: &str) -> Result<i32, String> {
    solve::<CSort>(input)
}

/// Process using Rust's built-in sort
pub fn process_rust_sort(input: &str) -> Result<i32, String> {
    solve::<NativeSort>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case("3 7", 4)]
    #[case("9 3", 6)]
    fn part_1_rust(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(process_rust_sort(input).unwrap(), expected);
    }

    #[rstest]
    #[case("3 7", 4)]
    #[case("9 3", 6)]
    fn part_1_c_qsort(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(process_c_qsort(input).unwrap(), expected);
    }

    #[test]
    fn test_both_agree() {
        assert_eq!(
            process_c_qsort(SAMPLE_INPUT).unwrap(),
            process_rust_sort(SAMPLE_INPUT).unwrap()
        );
    }
}
