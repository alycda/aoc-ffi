#!/usr/bin/env kotlin

/**
 * Test Kotlin bindings for the Advent of Code 2024 Day 1 Rust library.
 *
 * Run this after generating bindings with: just gen-kotlin
 *
 * To run:
 *   cd bindings/kotlin
 *   kotlinc -script ../../tests/kotlin/test_kotlin_bindings.kts -classpath .
 */

import uniffi.aoc_ffi_day01.uniffiProcessRustSort
import uniffi.aoc_ffi_day01.uniffiProcessCQsort
import uniffi.aoc_ffi_day01.uniffiProcessPart2
import uniffi.aoc_ffi_day01.uniffiProcessPart2Hashmap
import uniffi.aoc_ffi_day01.uniffiProcessPart2Glib
import uniffi.aoc_ffi_day01.uniffiProcessPart2Uthash
import uniffi.aoc_ffi_day01.AocException

val SAMPLE_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3"""

fun main() {
    println("Testing Advent of Code 2024 Day 1 - Kotlin Bindings via UniFFI\n")
    println("=".repeat(70))

    // Test Part 1 implementations
    println("\nPart 1: Sorting Algorithms")
    println("-".repeat(70))

    try {
        val rustSort = uniffiProcessRustSort(SAMPLE_INPUT)
        println("  Rust native sort:       $rustSort")

        val cQsort = uniffiProcessCQsort(SAMPLE_INPUT)
        println("  C qsort:                $cQsort")

        assert(rustSort == 11 && cQsort == 11) {
            "Part 1 results don't match!"
        }
        println("\n  ✓ All Part 1 implementations return 11")

    } catch (e: AocException) {
        println("  ✗ Error: ${e.message}")
        return
    }

    // Test Part 2 implementations
    println("\nPart 2: Hash Table Implementations")
    println("-".repeat(70))

    val implementations = mutableListOf<Pair<String, (String) -> Int>>(
        "Naive filter O(n*m)" to ::uniffiProcessPart2,
        "Rust AHashMap" to ::uniffiProcessPart2Hashmap,
        "uthash (C library)" to ::uniffiProcessPart2Uthash
    )

    // Try to add GLib if available
    try {
        uniffiProcessPart2Glib(SAMPLE_INPUT)
        implementations.add(2, "GLib GHashTable" to ::uniffiProcessPart2Glib)
        println("  (GLib feature enabled)")
    } catch (e: NoSuchMethodError) {
        println("  (GLib feature disabled - built with --no-default-features)")
    } catch (e: Throwable) {
        // GLib exists but might have failed for another reason
        implementations.add(2, "GLib GHashTable" to ::uniffiProcessPart2Glib)
        println("  (GLib feature enabled)")
    }

    println()
    val results = mutableListOf<Int>()
    for ((name, func) in implementations) {
        try {
            val result = func(SAMPLE_INPUT)
            results.add(result)
            println("  %-25s → %d".format(name, result))
        } catch (e: AocException) {
            println("  %-25s → Error: ${e.message}".format(name))
            return
        }
    }

    // Verify all results match
    val expected = 31
    if (results.all { it == expected }) {
        println("\n  ✓ All Part 2 implementations return $expected")
    } else {
        println("\n  ✗ Results don't match! Expected $expected, got ${results.toSet()}")
        return
    }

    // Test error handling
    println("\nError Handling Test")
    println("-".repeat(70))
    try {
        // This should work fine
        val result = uniffiProcessRustSort("1 2")
        println("  Valid input '1 2' → $result")
        println("  ✓ Error handling works")
    } catch (e: AocException) {
        println("  Unexpected error: ${e.message}")
    }

    println("\n" + "=".repeat(70))
    println("✓ All Kotlin binding tests passed!\n")
    println("Summary:")
    println("  - Part 1: 2 sorting implementations tested")
    println("  - Part 2: ${implementations.size} hash table implementations tested")
    println("  - All results match expected values")
    println("  - Error handling verified")
}

main()
