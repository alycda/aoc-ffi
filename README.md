# aoc-ffi

> **Learn Rust FFI through Advent of Code challenges**

A learning-focused project demonstrating Foreign Function Interface (FFI) patterns by implementing Advent of Code 2024 solutions in Rust and exposing them to Python, Kotlin, and Swift via [UniFFI](https://mozilla.github.io/uniffi-rs/).

This project was presented at Rust Los Angeles on January 28, 2026. See [slides.md](slides.md) for the full presentation.

## What This Project Does

Solves [Advent of Code 2024 Day 1](https://adventofcode.com/2024/day/1) ("Historian Hysteria") in Rust and demonstrates:

- **Multiple FFI approaches**: Compare C qsort, GLib hashtables, and uthash against Rust native implementations
- **Cross-language bindings**: Call Rust functions from Python, Kotlin, and Swift
- **Production practices**: Benchmarking, testing, cross-platform builds
- **Real FFI challenges**: Library paths, glibc conflicts, platform-specific binaries

### The Problem

- **Part 1**: Sort two lists of numbers and compute distance differences
- **Part 2**: Count frequency matches using hash tables

### Implementation Showcase

- **Part 1**: Compares C's qsort vs Rust's native sort
- **Part 2**: Demonstrates 5 different hash table implementations:
  - Naive O(n×m) filter
  - Rust HashMap
  - Rust ahash (fast non-crypto hash)
  - GLib's GHashTable (C library FFI)
  - uthash (header-only C library)

## Quick Start

### Prerequisites

This project uses [Nix](https://nixos.org/) for reproducible development environments:

```bash
# Enter Nix development shell
nix-shell

# Or use direnv for automatic loading
direnv allow
```

Alternatively, manually install:
- Rust (with cargo)
- Python 3 with pip
- JDK 17+ with Kotlin compiler
- Swift compiler (macOS)
- Just task runner

### Build & Test

```bash
# Build Rust library and run tests
just aoc-test

# Generate bindings for all languages
just uniffi-gen-all

# Test Python bindings
just uniffi-test-python

# Test Kotlin bindings
just uniffi-test-kotlin

# Test Swift bindings (macOS only)
just uniffi-test-swift

# Run benchmarks
just aoc-bench
```

## Project Structure

```
aoc-ffi/
├── aoc-2024-12-01/           # Main Rust FFI project (Day 1)
│   ├── src/
│   │   ├── lib.rs            # Rust implementation (442 lines)
│   │   ├── aoc_ffi_day01.udl # UniFFI interface definition
│   │   ├── uthash.h          # Header-only C hash table library
│   │   ├── uthash_wrapper.c  # C wrapper for uthash
│   │   └── uthash_wrapper.h  # C header for Rust FFI
│   ├── benches/              # Criterion benchmarks
│   ├── tests/                # Language binding tests
│   │   ├── python/test_python_bindings.py
│   │   ├── kotlin/test_kotlin_bindings.kts
│   │   └── swift/test_swift_bindings.swift
│   ├── bindings/             # Generated FFI bindings (gitignored)
│   │   ├── python/
│   │   ├── kotlin/
│   │   └── swift/
│   ├── Cargo.toml
│   └── build.rs              # UniFFI scaffolding generation
├── presentation/             # Markdown slide components
├── .devcontainer/            # VS Code devcontainer setup
├── justfile                  # Task orchestration (30+ recipes)
├── shell.nix                 # Nix environment setup
├── slides.md                 # Compiled presentation
└── CLAUDE.md                 # Claude Code development guide
```

## Available Commands

The [justfile](justfile) provides 30+ recipes organized by task:

### Rust Development

```bash
just aoc-test          # Run cargo tests
just aoc-bench         # Run criterion benchmarks
just aoc-run           # Run cargo run
```

### UniFFI Binding Generation

```bash
# Generate bindings (requires built library first)
just uniffi-gen-python    # Generate Python bindings
just uniffi-gen-kotlin    # Generate Kotlin bindings
just uniffi-gen-swift     # Generate Swift bindings
just uniffi-gen-all       # Generate all at once

# Test bindings
just uniffi-test-python   # Test Python bindings
just uniffi-test-kotlin   # Test Kotlin bindings
just uniffi-test-swift    # Test Swift (macOS only, skips on Linux)
just uniffi-test          # Python + Kotlin tests
just uniffi-test-all      # All language tests

# Maintenance
just clean-bindings       # Remove generated files
```

### Presentation

**Requirements:**
- [kitty terminal](https://sw.kovidgoyal.net/kitty/) (image / font-size support)
- [presenterm](https://github.com/mfontanini/presenterm) with [PR #826](https://github.com/mfontanini/presenterm/pull/826) (background color override support)

```bash
just present                     # Display slides
just present-with-speaker-notes  # tmux-based presenter mode
```

## Language Bindings

### Python

```python
from aoc_ffi_day01 import uniffi_process_rust_sort, uniffi_process_part_2_hashmap

# Part 1: Sort and compute distances
result = uniffi_process_rust_sort("3   4\n4   3\n2   5\n1   3\n3   9\n3   3")
print(result)  # Output: 11

# Part 2: Count frequency matches
result = uniffi_process_part_2_hashmap("3   4\n4   3\n2   5\n1   3\n3   9\n3   3")
print(result)  # Output: 31
```

### Kotlin

```kotlin
import uniffi.aoc_ffi_day01.*

// Part 1: C qsort implementation
val result = uniffiProcessCQsort("3   4\n4   3\n2   5\n1   3\n3   9\n3   3")
println(result)  // Output: 11

// Part 2: GLib hashtable (if available)
val result2 = uniffiProcessPart2Glib("3   4\n4   3\n2   5\n1   3\n3   9\n3   3")
println(result2)  // Output: 31
```

### Swift

```swift
import aoc_ffi_day01

// Part 1: Rust native sort
let result = uniffiProcessRustSort("3   4\n4   3\n2   5\n1   3\n3   9\n3   3")
print(result)  // Output: 11

// Part 2: uthash implementation
let result2 = uniffiProcessPart2Uthash("3   4\n4   3\n2   5\n1   3\n3   9\n3   3")
print(result2)  // Output: 31
```

## Cross-Platform Development

### Platform-Specific Libraries

When switching between macOS and Linux (devcontainer), rebuild bindings because they contain platform-specific libraries:

- **macOS**: Uses `.dylib` files
- **Linux**: Uses `.so` files
- **Windows**: Uses `.dll` files (untested)

```bash
# Clean all generated bindings
just clean-bindings

# Rebuild and test for current platform
just uniffi-test
```

### Swift on Linux

Swift tests are **currently skipped** on Linux due to glibc conflicts:
- Nix provides glibc 2.42, but system Swift requires glibc 2.39
- Swift bindings can be generated but not tested in devcontainer
- `just uniffi-test-swift` detects Linux + Nix and skips tests gracefully

### Devcontainer Isolation

The devcontainer uses Docker volumes to isolate platform-specific build artifacts:

- `.venv/` → Docker volume `aoc-ffi-venv` (Linux Python packages)
- `aoc-2024-12-01/target/` → Docker volume `aoc-ffi-target` (Linux Rust build cache)

This means:
- macOS `.venv` and `target/` live on your local filesystem
- Linux `.venv` and `target/` live in Docker volumes
- No conflicts when switching between environments
- Faster rebuilds (cached per platform)

## Architecture

### UniFFI Integration

This project uses [UniFFI](https://mozilla.github.io/uniffi-rs/) procedural macros for automatic binding generation:

```rust
#[uniffi::export]
pub fn uniffi_process_rust_sort(input: String) -> Result<i32, AocError> {
    // Implementation
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum AocError {
    #[error("Failed to parse input: {0}")]
    ParseError(String),
    #[error("Failed to allocate memory: {0}")]
    AllocationError(String),
}
```

### FFI Implementation Examples

#### C qsort Integration

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

#### GLib Hashtable FFI

```rust
use glib_sys::{g_hash_table_new, g_hash_table_insert, g_hash_table_lookup};
// Demonstrates FFI to external system libraries
```

#### uthash via cc Crate

```rust
// build.rs
cc::Build::new()
    .file("src/uthash_wrapper.c")
    .include("src")
    .opt_level(2)
    .compile("uthash_wrapper");
```

### Cargo Features

```toml
[features]
default = ["glib"]          # Include GLib by default
glib = ["glib-sys"]         # FFI to GLib's GHashTable
```

Use `--no-default-features` to exclude GLib (required for Swift on some platforms).

## Testing

All implementations are validated against the sample input from Advent of Code:

- **Expected Part 1 result**: 11
- **Expected Part 2 result**: 31

### Unit Tests

```bash
cargo test
```

10+ test cases validate all implementations and cross-validate that C qsort and Rust sort produce identical results.

### Integration Tests

Each language has comprehensive tests:

- **Python**: [tests/python/test_python_bindings.py](aoc-2024-12-01/tests/python/test_python_bindings.py) (116 lines)
- **Kotlin**: [tests/kotlin/test_kotlin_bindings.kts](aoc-2024-12-01/tests/kotlin/test_kotlin_bindings.kts) (120 lines)
- **Swift**: [tests/swift/test_swift_bindings.swift](aoc-2024-12-01/tests/swift/test_swift_bindings.swift) (108 lines)

## Benchmarking

Compare performance of different implementations:

```bash
just aoc-bench
```

Uses [Criterion](https://github.com/bheisler/criterion.rs) to benchmark:
- C qsort vs Rust sort
- Naive O(n×m) vs HashMap vs ahash vs GLib vs uthash

Requires `benches/REAL_INPUT.txt` (gitignored) for realistic benchmarking.

## Dependencies

### Core Rust

- **uniffi 0.28.3** - FFI framework
- **ahash 0.8** - Fast non-crypto hashing
- **glib-sys 0.20** - GLib FFI bindings (optional)
- **thiserror 1.0** - Error handling
- **criterion 0.5** - Benchmarking
- **rstest 0.26** - Parametrized testing
- **cc 1** - C compiler for build scripts

### External Tools

- **uniffi-bindgen 0.28.3** - Code generator (Python pip)
- **Kotlin** - JVM language for bindings
- **Swift** - iOS/macOS language for bindings
- **JNA 5.13.0** - Java Native Access library
- **Nix** - Reproducible environments (optional)

## Learning Resources

### Presentation

The [slides.md](slides.md) file contains the full presentation delivered at Rust Los Angeles, including:

- Why Rust for FFI
- Advent of Code overview
- Day 1 solution deep dive
- **Lessons learned** (most valuable section)
- What worked well
- Q&A and errata

### Key Lessons Documented

The presentation documents real challenges encountered:

- Cross-platform library extension handling
- Nix + Python environment isolation
- JNA library path configuration
- Docker volume isolation strategy
- UniFFI API surprises and inspection techniques
- Build system complexity management

See [CLAUDE.md](CLAUDE.md) for development guidance specific to Claude Code sessions.

## Version Control

This project uses [Jujutsu](https://github.com/martinvonz/jj) (jj) for version control, but git is also supported.

```bash
# View status
jj st

# View log
jj log
```

## Contributing

This is a learning project and reference implementation. Feel free to:

- Explore the code and presentation materials
- Use it as a template for your own FFI projects
- Open issues for questions or suggestions
- Submit PRs for improvements

## Acknowledgments

- [Advent of Code](https://adventofcode.com/) by Eric Wastl
- [UniFFI](https://mozilla.github.io/uniffi-rs/) by Mozilla
- [uthash](https://troydhanson.github.io/uthash/) by Troy D. Hanson
- Rust Los Angeles community

---

**Built with** ❤️ **and Rust FFI**
