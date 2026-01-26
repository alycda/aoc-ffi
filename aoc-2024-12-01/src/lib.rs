//! 2024 Day 1: Historian Hysteria - FFI Benchmarking Library
//!
//! NOTE: This example is intentionally contrived - Rust's built-in sort is both
//! safer and faster. The point is to demonstrate FFI mechanics (extern "C",
//! function pointers, unsafe) with a familiar algorithm before tackling more
//! complex FFI scenarios.

use std::ffi::c_void;
use std::os::raw::c_int;

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

/// Process using C qsort
pub fn process_c_qsort(input: &str) -> Result<i32, String> {
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

/// Process using Rust's built-in sort
pub fn process_rust_sort(input: &str) -> Result<i32, String> {
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

// ============================================================================
// Part 2: Hash Table Implementations
// ============================================================================

use std::collections::HashMap;

/// Part 2 using Rust HashMap with fold
///
/// This approach:
/// 1. Builds a frequency map of the right list using fold
/// 2. For each number in left, multiply it by its count in right
pub fn process_part_2_hashmap(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    // Build frequency map using fold
    let counts: HashMap<i32, usize> = right.iter()
        .fold(HashMap::new(), |mut acc, &n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });

    Ok(left
        .iter()
        .map(|&n| n * *counts.get(&n).unwrap_or(&0) as i32)
        .sum())
}

// Using glib-sys - GLib hash table for frequency counting
#[cfg(feature = "glib")]
use glib_sys::{
    g_hash_table_new, g_hash_table_destroy, g_hash_table_insert,
    g_hash_table_lookup, g_direct_hash, g_direct_equal, gpointer
};

/// Build frequency map using GLib's GHashTable
/// GHashTable is GLib's hash map implementation (similar to HashMap in Rust)
#[cfg(feature = "glib")]
unsafe fn glib_build_freq_map(arr: &[i32]) -> *mut glib_sys::GHashTable {
    unsafe {
        // Create hash table with direct hash (for integer keys stored as pointers)
        let table = g_hash_table_new(
            Some(g_direct_hash),    // hash function for integer keys
            Some(g_direct_equal)     // equality function for integer keys
        );

        // Count frequencies
        for &value in arr {
            // GLib stores keys/values as void pointers (gpointer)
            let key = value as gpointer;

            // Look up current count (stored as pointer)
            let current = g_hash_table_lookup(table, key);
            let count = if current.is_null() {
                0
            } else {
                current as isize
            };

            // Store incremented count
            let new_count = (count + 1) as gpointer;
            g_hash_table_insert(table, key, new_count);
        }

        table
    }
}

#[cfg(feature = "glib")]
unsafe fn glib_get_freq(table: *mut glib_sys::GHashTable, value: i32) -> i32 {
    unsafe {
        let key = value as gpointer;
        let result = g_hash_table_lookup(table, key);

        if result.is_null() {
            0
        } else {
            result as i32
        }
    }
}

/// Part 2 using GLib's GHashTable via glib-sys FFI
///
/// This demonstrates using GLib's hash table from C.
/// GHashTable is a widely-used hash table implementation in the C ecosystem.
///
/// Enabled with the "glib" feature (default).
/// To build without glib: cargo build --no-default-features
#[cfg(feature = "glib")]
pub fn process_part_2_glib(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    unsafe {
        // Build frequency map using GLib
        let freq_table = glib_build_freq_map(&right);

        // Calculate result
        let mut result = 0;
        for &n in &left {
            let count = glib_get_freq(freq_table, n);
            result += n * count;
        }

        // Clean up GLib hash table
        g_hash_table_destroy(freq_table);

        Ok(result)
    }
}

// ============================================================================
// uthash - Header-only C hash table library via cc + extern "C"
// ============================================================================

// Opaque type representing the uthash hash table (struct hash_entry*)
#[repr(C)]
struct HashEntry {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    fn uthash_build_frequency_map(arr: *const i32, len: usize) -> *mut HashEntry;
    fn uthash_lookup(hash_table: *mut HashEntry, key: i32) -> i32;
    fn uthash_destroy(hash_table: *mut HashEntry);
}

/// Part 2 using uthash (C header-only hash table library)
///
/// uthash is a popular C hash table implementation using macros.
/// This demonstrates FFI with a C library compiled via the `cc` crate.
pub fn process_part_2_uthash(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    unsafe {
        // Build frequency map using uthash
        let hash_table = uthash_build_frequency_map(
            right.as_ptr(),
            right.len()
        );

        // Calculate result
        let mut result = 0;
        for &n in &left {
            let count = uthash_lookup(hash_table, n);
            result += n * count;
        }

        // Clean up
        uthash_destroy(hash_table);

        Ok(result)
    }
}

/// Part 2 using naive filter approach (original - for comparison)
///
/// This is O(n*m) - for each left element, scan entire right list
pub fn process_part_2(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    Ok(left
        .iter()
        .map(|n| n * right.iter().filter(|&x| x==n).count() as i32)
        .sum())
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
    fn test_part_1_agree() {
        assert_eq!(
            process_c_qsort(SAMPLE_INPUT).unwrap(),
            process_rust_sort(SAMPLE_INPUT).unwrap()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(SAMPLE_INPUT).unwrap(), 31);
    }

    #[test]
    fn test_part_2_hashmap() {
        assert_eq!(process_part_2_hashmap(SAMPLE_INPUT).unwrap(), 31);
    }

    #[test]
    #[cfg(feature = "glib")]
    fn test_part_2_glib() {
        assert_eq!(process_part_2_glib(SAMPLE_INPUT).unwrap(), 31);
    }

    #[test]
    fn test_part_2_uthash() {
        assert_eq!(process_part_2_uthash(SAMPLE_INPUT).unwrap(), 31);
    }

    #[test]
    fn test_all_part_2_agree() {
        let naive = process_part_2(SAMPLE_INPUT).unwrap();
        let hashmap = process_part_2_hashmap(SAMPLE_INPUT).unwrap();
        #[cfg(feature = "glib")]
        let glib = process_part_2_glib(SAMPLE_INPUT).unwrap();

        let uthash = process_part_2_uthash(SAMPLE_INPUT).unwrap();

        assert_eq!(naive, hashmap);
        #[cfg(feature = "glib")]
        assert_eq!(hashmap, glib);
        assert_eq!(hashmap, uthash);
        assert_eq!(naive, 31);
    }
}
