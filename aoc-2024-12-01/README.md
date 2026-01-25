# Advent of Code 2024 Day 1: FFI Sorting Comparison

A learning project that demonstrates Foreign Function Interface (FFI) between Rust, C, and C++ by implementing the same sorting algorithm in three different ways.

## Overview

This project solves Advent of Code 2024 Day 1 using three different sorting implementations:

1. **C `qsort`** - Classic C standard library sorting
2. **Rust native `sort`** - Rust's built-in sorting
3. **C++ `std::sort`** - C++ STL sorting via autocxx

## Project Structure

```
aoc-2024-12-01/
├── src/
│   ├── main.rs         # Rust implementation with all three approaches
│   └── cpp_sort.h      # C++ wrapper for std::sort
├── main.cpp            # Standalone C++ comparison (qsort vs std::sort)
├── Makefile            # Build C++ standalone program
├── build.rs            # autocxx build configuration
├── Cargo.toml          # Rust dependencies
└── README.md           # This file
```

## Building and Running

### Prerequisites

The project uses Nix for dependency management. Required packages:
- `gcc` - C++ compiler
- `gnumake` - Build tool
- `clang` - Required by autocxx
- `cargo`/`rustc` - Rust toolchain

### Using Just (Recommended)

From the repository root:

```bash
# Run Rust version (all three approaches)
just rust-run

# Build and run C++ standalone comparison
just cpp-build
just cpp-run

# Or run both
just compare
```

### Manual Build

```bash
# Rust
cd aoc-2024-12-01
cargo run --release

# C++
cd aoc-2024-12-01
make
./main
```

## Technical Details

### 1. C qsort via FFI

**File:** [src/main.rs:30-48](src/main.rs#L30-L48)

```rust
unsafe extern "C" {
    fn qsort(
        base: *mut c_void,
        num: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int
    );
}
```

**Key Points:**
- Direct FFI to libc's `qsort`
- Requires unsafe block
- Function pointer prevents inlining
- Slowest due to runtime indirection

### 2. Rust Native Sort

**File:** [src/main.rs:87-99](src/main.rs#L87-L99)

```rust
left.sort();
right.sort();
```

**Key Points:**
- Uses pattern-defeating quicksort (pdqsort)
- Fully optimized at compile time
- Type-safe, no unsafe required
- Fastest due to monomorphization

### 3. C++ std::sort via autocxx

**Files:**
- C++ wrapper: [src/cpp_sort.h](src/cpp_sort.h)
- Rust binding: [src/main.rs:14-21](src/main.rs#L14-L21)
- Build config: [build.rs](build.rs)

```rust
include_cpp! {
    #include "cpp_sort.h"
    safety!(unsafe)
    generate!("cpp_sorting::sort_i32_array")
}
```

**Key Points:**
- Uses autocxx for automatic binding generation
- Requires libclang at build time
- C++ introsort algorithm (hybrid approach)
- Still requires unsafe for FFI call

## Performance Comparison

From the C++ standalone benchmark ([main.cpp](main.cpp)):

| Array Size | C qsort | C++ std::sort | Speedup |
|------------|---------|---------------|---------|
| 100        | 0.036ms | 0.033ms       | 1.12x   |
| 1,000      | 0.294ms | 0.167ms       | 1.76x   |
| 10,000     | 1.697ms | 1.288ms       | 1.32x   |
| 100,000    | 18.52ms | 15.30ms       | 1.21x   |

**Why is std::sort faster?**
- Template inlining (no function pointer calls)
- Better algorithm (introsort vs quicksort)
- Compile-time optimization

## Learning Resources

- **C qsort:** https://www.tutorialspoint.com/c_standard_library/c_function_qsort.htm
- **C++ std::sort:** https://www.geeksforgeeks.org/cpp/c-qsort-vs-c-sort/
- **autocxx:** https://docs.rs/autocxx/latest/autocxx/

## Notes

- This is a **learning exercise**, not production code
- The Rust native sort is typically the best choice for Rust projects
- FFI has overhead - only use when necessary (e.g., calling existing C/C++ libraries)
- autocxx simplifies C++ FFI but adds build complexity

## License

Learning/educational purposes - part of AOC 2024
