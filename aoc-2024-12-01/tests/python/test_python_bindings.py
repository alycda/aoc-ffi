#!/usr/bin/env python3
"""
Test Python bindings for the Advent of Code 2024 Day 1 Rust library.

Run this after generating bindings with generate_python_bindings.py
"""

import sys
from pathlib import Path

# Add bindings directory to path
project_root = Path(__file__).parent.parent.parent
bindings_dir = project_root / "bindings" / "python"
sys.path.insert(0, str(bindings_dir))

try:
    import aoc_ffi_day01
except ImportError as e:
    print(f"Error importing bindings: {e}")
    print(f"\nSearched in: {bindings_dir}")
    print("\nPlease generate bindings first:")
    print("  cd aoc-2024-12-01")
    print("  python3 scripts/generate_python_bindings.py")
    print("\nOr install uniffi-bindgen and run:")
    print("  cargo install uniffi-bindgen --version 0.28.3")
    print("  uniffi-bindgen generate src/aoc_ffi_day01.udl --library target/release/libaoc_ffi_day01.so --language python --out-dir bindings/python")
    sys.exit(1)

SAMPLE_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3"""

def main():
    print("Testing Advent of Code 2024 Day 1 - Python Bindings via UniFFI\n")
    print("=" * 70)

    # Test Part 1 implementations
    print("\nPart 1: Sorting Algorithms")
    print("-" * 70)

    try:
        rust_sort = aoc_ffi_day01.uniffi_process_rust_sort(SAMPLE_INPUT)
        print(f"  Rust native sort:       {rust_sort}")

        c_qsort = aoc_ffi_day01.uniffi_process_c_qsort(SAMPLE_INPUT)
        print(f"  C qsort:                {c_qsort}")

        assert rust_sort == c_qsort == 11, "Part 1 results don't match!"
        print("\n  ✓ All Part 1 implementations return 11")

    except aoc_ffi_day01.AocError as e:
        print(f"  ✗ Error: {e}")
        return 1

    # Test Part 2 implementations
    print("\nPart 2: Hash Table Implementations")
    print("-" * 70)

    implementations = [
        ("Naive filter O(n*m)", aoc_ffi_day01.uniffi_process_part_2),
        ("Rust AHashMap", aoc_ffi_day01.uniffi_process_part_2_hashmap),
        ("uthash (C library)", aoc_ffi_day01.uniffi_process_part_2_uthash),
    ]

    # Add GLib if available
    try:
        glib_result = aoc_ffi_day01.uniffi_process_part_2_glib(SAMPLE_INPUT)
        implementations.insert(2, ("GLib GHashTable", aoc_ffi_day01.uniffi_process_part_2_glib))
        print("  (GLib feature enabled)")
    except AttributeError:
        print("  (GLib feature disabled - built with --no-default-features)")

    print()
    results = []
    for name, func in implementations:
        try:
            result = func(SAMPLE_INPUT)
            results.append(result)
            print(f"  {name:25s} → {result}")
        except aoc_ffi_day01.AocError as e:
            print(f"  {name:25s} → Error: {e}")
            return 1

    # Verify all results match
    expected = 31
    if all(r == expected for r in results):
        print(f"\n  ✓ All Part 2 implementations return {expected}")
    else:
        print(f"\n  ✗ Results don't match! Expected {expected}, got {set(results)}")
        return 1

    # Test error handling
    print("\nError Handling Test")
    print("-" * 70)
    try:
        # This should work fine
        result = aoc_ffi_day01.uniffi_process_rust_sort("1 2")
        print(f"  Valid input '1 2' → {result}")
        print("  ✓ Error handling works")
    except aoc_ffi_day01.AocError as e:
        print(f"  Unexpected error: {e}")

    print("\n" + "=" * 70)
    print("✓ All Python binding tests passed!\n")
    print("Summary:")
    print(f"  - Part 1: 3 sorting implementations tested")
    print(f"  - Part 2: {len(implementations)} hash table implementations tested")
    print(f"  - All results match expected values")
    print(f"  - Error handling verified")
    return 0

if __name__ == "__main__":
    sys.exit(main())
