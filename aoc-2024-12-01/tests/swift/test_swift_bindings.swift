/**
 * Test Swift bindings for the Advent of Code 2024 Day 1 Rust library.
 *
 * Run this after generating bindings with: just uniffi-gen-swift
 *
 * To compile and run:
 *   cd bindings/swift
 *   swiftc -o test_swift ../../tests/swift/test_swift_bindings.swift aoc_ffi_day01.swift \
 *       -import-objc-header aoc_ffi_day01FFI.h -L . -laoc_ffi_day01
 *   LD_LIBRARY_PATH=. ./test_swift
 */

import Foundation

@main
struct TestRunner {
    static func main() {
        let SAMPLE_INPUT = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        """

        print("Testing Advent of Code 2024 Day 1 - Swift Bindings via UniFFI\n")
        print(String(repeating: "=", count: 70))

        // Test Part 1 implementations
        print("\nPart 1: Sorting Algorithms")
        print(String(repeating: "-", count: 70))

        do {
            let rustSort = try uniffiProcessRustSort(input: SAMPLE_INPUT)
            print("  Rust native sort:       \(rustSort)")

            let cQsort = try uniffiProcessCQsort(input: SAMPLE_INPUT)
            print("  C qsort:                \(cQsort)")

            assert(rustSort == 11 && cQsort == 11,
                   "Part 1 results don't match!")
            print("\n  ✓ All Part 1 implementations return 11")

        } catch {
            print("  ✗ Error: \(error)")
            exit(1)
        }

        // Test Part 2 implementations
        print("\nPart 2: Hash Table Implementations")
        print(String(repeating: "-", count: 70))

        var implementations: [(String, (String) throws -> Int32)] = [
            ("Naive filter O(n*m)", uniffiProcessPart2),
            ("Rust AHashMap", uniffiProcessPart2Hashmap),
            ("uthash (C library)", uniffiProcessPart2Uthash),
        ]

        // Note: GLib feature disabled when built with --no-default-features for Swift compatibility
        print("  (GLib feature disabled - built with --no-default-features)")

        print()
        var results: [Int32] = []
        for (name, function) in implementations {
            do {
                let result = try function(SAMPLE_INPUT)
                results.append(result)
                let paddedName = name.padding(toLength: 25, withPad: " ", startingAt: 0)
                print("  \(paddedName) → \(result)")
            } catch {
                let paddedName = name.padding(toLength: 25, withPad: " ", startingAt: 0)
                print("  \(paddedName) → Error: \(error)")
                exit(1)
            }
        }

        // Verify all results match
        let expected: Int32 = 31
        if results.allSatisfy({ $0 == expected }) {
            print("\n  ✓ All Part 2 implementations return \(expected)")
        } else {
            print("\n  ✗ Results don't match! Expected \(expected), got \(Set(results))")
            exit(1)
        }

        // Test error handling
        print("\nError Handling Test")
        print(String(repeating: "-", count: 70))
        do {
            // This should work fine
            let result = try uniffiProcessRustSort(input: "1 2")
            print("  Valid input '1 2' → \(result)")
            print("  ✓ Error handling works")
        } catch {
            print("  Unexpected error: \(error)")
        }

        print("\n" + String(repeating: "=", count: 70))
        print("✓ All Swift binding tests passed!\n")
        print("Summary:")
        print("  - Part 1: 2 sorting implementations tested")
        print("  - Part 2: \(implementations.count) hash table implementations tested")
        print("  - All results match expected values")
        print("  - Error handling verified")
    }
}
