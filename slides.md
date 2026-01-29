---
theme:
    name: catppuccin-mocha
    override:
        footer:
            style: template
            left: "Learning in Public : AOC FFI Playground"
            center: github.com/alycda/aoc-ffi
---



<!-- font_size: 7 -->

![](./presentation/img/corro.png)

# (ab)Using Advent of Code as an FFI Playground
## [Learning in Public]

### @ Rust Los Angeles
#### Santa Monica, CA
##### 2026-01-28

###### Alyssa Evans

<!-- speaker_note: |
    Hello_ Rust LA! 

    My name is Alyssa,
        and tonight I want to share with you part of my journey in Rust:
    
    Learning in Public,
        through Advent of Code, 
            Rust FFI Edition


    Quick Disclaimer:
        this is my first public talk

        after 20 years I've broken _many_ things on the internet and let's just hope that today isn't one of those days

        I now have about 20s of public speaking experience so go easy on me


    Before we dive in: huge thanks to Sean, Ross, and the whole Lawrence Harvey crew for organizing this.


    So who am I and why am I talking about FFI in Rust?



    [30s]
 -->

<!-- no_footer -->

<!-- end_slide -->



<!-- font_size: 7 -->

Who am I?
===

<!-- slide_background_color: black -->

![image:w:54%](./presentation/img/1200x800black.jpg)

<!-- font_size: 3 -->
<!-- alignment: center -->
Staff Software Engineer at [Ditto](https://ditto.com) 

<!-- new_line -->

<!-- font_size: 2 -->
<span style="color: #000;">and Motorcycle Road Racer</span>

![](./presentation/img/Ditto.png)

<!-- speaker_note: |
    I recently joined Ditto as a Staff Software Engineer, 

        working on Rust FFI SDKS (for platforms like iOS, Android and a dozen more). FFI is literally my day job.



    Before Ditto, I spent 6 years in Free Ad-Supported Streaming TV (FAST) 

        building Web Applications for Connected TVs and game consoles. 

        Then I became _that engineer_ who kept pushing to adopt Rust.



    [15-20s]
 -->

<!-- end_slide -->



<!-- font_size: 7 -->

Who am I?
===

<!-- slide_background_color: black -->

![image:w:50%](./presentation/img/CaliPhoto.jpg)

<!-- font_size: 3 -->

<!-- alignment: center -->
Staff Software Engineer at [Ditto](https://ditto.com) 

<!-- new_line -->

<!-- font_size: 2 -->
and Motorcycle Road Racer

![](./presentation/img/Ditto.png)

<!-- speaker_note: |
    and I race motorcycles for fun!


    I have a need for speed so if I start talking too fast, I'm sorry in advance! 
        
        But you can also read the transcript of this presentation on Github.


    Alright, credentials established. Let's talk about why I chose Rust.



    [10-15s]
 -->

<!-- end_slide -->



<!-- font_size: 7 -->

Why Rust?
===

<!-- new_line -->

![image:w:65%](./presentation/img/cuddlyferris.png)

<!-- speaker_note: |
    I'm sure I don't have to convince anyone here but here's why Rust specifically made my journey possible.

    A few years ago I was a TypeScript engineer working on a Video Player Integration 
        when a Partner company asked us to rewrite our code from JS in the Browser 
        to native C++ for embedded Linux CTVS. 
        
    I tried learning C++ on my own but I just couldn't make it work, especially not fast enough to meet the partner's desired timeline.

    My colleague (and friend) on this project suggested Rust. 
        He'd mentioned it before and I recently saw a company hackathon project 
        showing how fast it was compared to TypeScript, Go and other languages, 
        so I decided to try to learn Rust instead of C++.


    Immediately I was hooked on the language, ecosystem and community, 
        especially the quality of tutorials available; 
        
        and Rust made it possible for a TypeScript engineer 
        to transition into Systems Programming at lightspeed 

            (with a LOT of help from Claude along the way to explain concepts and syntax, more on that later).


    That transition relied on creative learning tools. ~~Which brings us to Advent of Code.~~

    So let me tell you about how Advent of Code became my playground(s) for Rust
        (and later FFI and then CRDTs)



    [50-60s]
 -->

<!-- end_slide -->



<!-- font_size: 7 -->

Advent of Code → Rust FFI
===

<!-- slide_background_color: 0f0f23 --> 

<!-- new_lines: 2 -->

![image:w:50%](./presentation/img/gru.jpg)

<!-- speaker_note: |
    First of all, I hate LeetCode. 
    
    I hate HackerRank. 
    
    Word problems make me miserable. 
    
    I'd never even done Advent of Code before.


    But after months of therapy sessions with the Rust compiler—fighting mutable strings across threads, 

        wrestling with lifetimes—I needed wins. 

        Small victories. 


    And I'm competitive, remember the motorcycle?



    [20s]
 -->

<!-- end_slide -->

I like winning!
===

<!-- slide_background_color: black --> 

![](./presentation/img/femmewalla.jpg)

<!-- speaker_note: |
    [5s] -->

<!-- no_footer -->
<!-- end_slide -->



<!-- font_size: 7 -->

Advent of Code → Rust FFI
===

<!-- slide_background_color: 0f0f23 --> 

![image:w:90%](./presentation/img/aoc.gif)

<!-- speaker_note: |
    So that December, I joined the company's AoC challenge. 

        Who doesn't want to save Christmas?


    I Took 3rd place.

        Behind a Java wizard and a Pythonista. 

        They'd been coding for decades in those languages. 
        
        I'd been writing Rust for months.


    Fast-forward to November 2025: 
        I'm onboarding at Ditto, learning their FFI patterns for a dozen platforms. 
        
        And I think: "I learned Rust through AoC. What if I could systematically reinforce the ffi I recently learned and deployed the exact same way?"


    Went back to those 2024 puzzles. Started experimenting on GitHub



    [30s]
 -->

<!-- end_slide -->



<!-- font_size: 7 -->

Advent of Code → Rust FFI
===

<!-- slide_background_color: 0f0f23 --> 

<!-- font_size: 6 -->

<!-- new_line -->

## Step-by-step exploration
of Foreign Function Interfaces in Rust

<!-- incremental_lists: true -->
1. Pure Rust solution
2. C FFI with qsort
3. GLib & uthash integration
4. Benchmarking with Criterion
5. UniFFI (python/kotlin/swift)
<!-- incremental_lists: false -->

<!-- new_line -->

### Bonus
* Zero-cost abstractions



<!-- speaker_note: |
    
    So here's what I did: 


    For each puzzle, I'm going to progressively complicate the solution:

        [next]
        1. First we'll start with pure Rust to establish an idiomatic baseline

        [next]
        2. Then we'll replace part of the code with a simple call to C

        [next]
        3. Then we'll add complexity by integrating a real C library with headers 

        [next]
        4. Then we'll benchmark with both sample and full puzzle inputs

        [next]
        5. Then we'll get silly and call C from Rust into higher level languages (like Python) via UniFFI 


    [next]
    Later I'll show some zero-cost abstractions in action



    Now Let's start with 2024 Day 1...



    [30s]
-->

<!-- end_slide -->



<!-- font_size: 7 -->

Day 1: Historian Hysteria
===

<!-- speaker_note: |
    Day 1 is just a simple 2-list problem. 
        [next] We need to parse, 
            [next] sort 
                [next] and then find the absolute difference.

    [next: input/output]

    [next: parse fn]

    so here's our baseline established in rust.


    but what if we want to use a different sorting algorithm? 
        well rust has sort and sort_unstable, but

        let's use C's Quicksort because why not!



    [20-30s]
 -->

<!-- font_size: 2 -->

## Problem

<!-- incremental_lists: true -->
**Given two lists of numbers:**
* Parse input into two separate lists
* Sort both lists
* Calculate sum of absolute differences
<!-- incremental_lists: false -->

```rust
Input:
3   4
4   3
2   5
Output: Sum of |3-3| + |4-4| + |2-5| = 3
```

<!-- pause -->

### Parse Input

```rust
fn unzip(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines()
            .map(|line| {
                let nums = line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (nums[0], nums[1])
            })
            .unzip()
}
```

<!-- pause -->

### Solution

```rust
fn process(input: &str) -> Result<String, String> {
    let (mut left, mut right) = unzip(input);

    left.sort(); 
    right.sort(); 

    let output = left.iter().zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    Ok(output.to_string())
}
```

<!-- end_slide -->



<!-- font_size: 7 -->

Enter C FFI
===

<!-- speaker_note: |
    so we need to declare the fn signature in an extern "C" block

    [next]

    and we need to tell C how to compare items, otherwise it doesn't know what to do with the bytes it's given. strcmp? integers, floats?

    and Actually, there's a bug in this code, can you find it?



    [15s]

    ---

    yes I know that we can just import from libc but I wanted to show manual implementation without a (large?) dependency
 -->

<!-- font_size: 2 -->

## **Why use C's qsort?**
* Learn FFI basics
* Compare performance with Rust
* Demonstrate unsafe code patterns

<!-- new_line -->

```rust
use std::ffi::c_void;
use std::os::raw::c_int;

unsafe extern "C" {
    fn qsort(
        base: *mut c_void,
        num: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void,
                                     *const c_void) -> c_int,
    );
}
```

<!-- pause -->

<!-- new_lines: 4 -->

### Compare Fn

**C expects a comparison function:**

```rust
unsafe extern "C" fn compare_i32(
    a: *const c_void,
    b: *const c_void
) -> c_int {
    unsafe {
        let a = *(a as *const i32);
        let b = *(b as *const i32);
        a - b
    }
}
```

<!-- end_slide -->



<!-- font_size: 7 -->

BUG!
===

<!-- font_size: 2 -->

### Compare Fn: Can You Spot the Bug?

**Common but dangerous C pattern:**

```rust
unsafe extern "C" fn compare_i32(
    a: *const c_void,
    b: *const c_void
) -> c_int {
    unsafe {
        let a = *(a as *const i32);
        let b = *(b as *const i32);
        a - b  // 🐛 What could go wrong?
    }
}
```

<!-- pause -->

**Hint:** Think about extreme values...

<!-- pause -->

<!-- new_line -->

#### The Problem: Integer overflow!
* If `a = i32::MAX` and `b = i32::MIN`
* Subtraction panics (debug mode) or wraps (release mode)
* Results in incorrect comparison behavior

<!-- new_line -->

**Better approaches:**

```rust
// Explicit C idiom
if a < b { -1 } else if a > b { 1 } else { 0 }

// Or use Rust's cmp
a.cmp(&b) as c_int // but this defeats the purpose of manual FFI implementation for this demo
```

<!-- speaker_note: |
    that's right, integer overflow


    the Rust compiler can't help us here because we are in an UNSAFE block 
        and we solemnly swore that we are up to no good! 

    so we need to either write out this if/else statement 
        or just use the Compare method (and cast) but again the compiler won't tell us about it.


    [15s]
    
    -->

<!-- end_slide -->



<!-- font_size: 7 -->

Enter C FFI
===

<!-- font_size: 2 -->

### Wrapper Fn

**Why wrap it?**
* Encapsulates unsafe FFI call
* Provides ergonomic Rust API (`&mut Vec<i32>`)
* Handles pointer/size boilerplate
* Contains `unsafe` to single location

<!-- new_line -->

```rust
fn c_qsort(vec: &mut Vec<i32>) {
    unsafe {
        qsort(
            vec.as_mut_ptr() as *mut c_void,
            vec.len(),
            std::mem::size_of::<i32>(),
            compare_i32
        );
    }
}
```

<!-- new_lines: 2 -->

### Diff
```diff
- left.sort();
- right.sort();
+ c_qsort(&mut left);
+ c_qsort(&mut right);
```

<!-- new_lines: 2 -->

#### **Key points:**
* `extern "C"` ABI compatibility
* Raw pointer manipulation
* Unsafe block required

<!-- speaker_note: |
    Now we need a wrapper fn to contain the unsafe block to a single location.

    C expects a raw pointer, sizes and a compare fn, 
        Rust has fat pointers & type safety

    so we make a safe fn 🤞🏼 for a clean API swap

    --- but we are making a promise to the compiler (that we know what we are doing) and we MUST keep it


    [20-30s]

    -->

<!-- end_slide -->



<!-- font_size: 7 -->

Benchmarks
===

<!-- speaker_note: |

    Here are some early benchmark results.
        No surprise here, C is definitely slower due to FFI overhead, 
        especially on the sample input.

    [next]

    But we test/bench early and often to suppress assumptions with facts about our actual data & use cases.


    Now, on to part 2 with some more interesting results that warrant this experimentation


    [20-30s]

    -->

<!-- font_size: 2 -->

### Sample Input (criterion)

<!-- new_line -->

```
process c qsort         time:   [238.81 ns 239.68 ns 240.62 ns]

process rust sort       time:   [178.21 ns 178.88 ns 179.67 ns]
```

<!-- pause -->

<!-- new_line -->

#### re-run

<!-- new_line -->

```
     Running benches/aoc_bench.rs (target/release/deps/aoc_bench-e5fedc316ff25a5f)
Gnuplot not found, using plotters backend
process c qsort         time:   [230.71 ns 231.45 ns 232.23 ns]
                        change: [-3.6254% -3.1719% -2.6822%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

process rust sort       time:   [176.38 ns 177.38 ns 178.44 ns]
                        change: [-1.5001% -1.0515% -0.5724%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

```

<!-- pause -->

<!-- new_lines: 2 -->

**Rust's sort is ~25% faster** on this sample input

<!-- new_line -->

* C's `qsort` uses function pointer indirection (no inlining)
* Rust's `.sort()` monomorphizes the comparator
* FFI call overhead adds up per comparison

<!-- end_slide -->



<!-- font_size: 7 -->

Part 2: Similarity Score
===

<!-- speaker_note: |

    Same input, but different problem.

    [next]

    here I'm showing a hashmap, but there's a naive solution that wins the benchmarks, 
        again on the sample input.


    but let's move on to a more complex integration with a real C library


    [15-20s]

    ---

    Same input, different problem. Now we're counting occurrences and calculating a weighted sum.

    there's a shorter naive solution that I have benchmarked but let's move onto different hashmap implementations from C

    "What if we need a real C library?"

    -->

<!-- font_size: 2 -->

## Problem

<!-- new_line -->

**Given the same two lists:**
* For each number in the left list, count how many times it appears in the right list
* Multiply the number by its count
* Sum all the products

<!-- new_line -->

```rust
Input:
3   4        3 appears 3x in right → 3 * 3 = 9
4   3        4 appears 1x in right → 4 * 1 = 4
2   5        2 appears 0x in right → 2 * 0 = 0

Output: 9 + 4 + 0 = 13
```

<!-- pause -->

<!-- new_lines: 2 -->

### HashMap Solution — O(n) instead of O(n*m)

```rust
use std::collections::HashMap;

pub fn process_part_2_hashmap(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    let counts: HashMap<i32, usize> = right.iter()
        .fold(HashMap::new(), |mut acc, &n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });

    Ok(left.iter()
        .map(|&n| n * *counts.get(&n).unwrap_or(&0) as i32)
        .sum())
}
```

<!-- pause -->

<!-- new_line -->

**But Big-O doesn't tell the whole story** — hash overhead can dominate on small inputs

<!-- end_slide -->



<!-- font_size: 7 -->

GLib Hash Table FFI
===

<!-- speaker_note: |

    GLib hash tables - everywhere in Linux.

    What changes with real C libraries:

    - integers as void pointers (gpointer type coercion)
    - manual memory management - we call destroy, no Drop trait
    - system dependencies via pkg-config

    Feature-flagged because not everyone has GLib installed.


    [30s]

 -->

<!-- font_size: 2 -->

### Using C's GHashTable from Rust

GLib provides a widely-used hash table implementation in C.
Let's use it via FFI with `glib-sys`:

```rust
use glib_sys::{g_hash_table_new, g_hash_table_lookup,
               g_hash_table_insert, g_hash_table_destroy,
               g_direct_hash, g_direct_equal, gpointer};

unsafe fn glib_build_freq_map(arr: &[i32]) -> *mut GHashTable {
    let table = g_hash_table_new(Some(g_direct_hash), Some(g_direct_equal));

    for &value in arr {
        let key = value as gpointer;  // Cast i32 to void pointer
        let current = g_hash_table_lookup(table, key);
        let count = if current.is_null() { 0 } else { current as isize };

        g_hash_table_insert(table, key, (count + 1) as gpointer);
    }
    table
}

fn process_part_2_glib(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    unsafe {
        let freq_table = glib_build_freq_map(&right);
        let result = left.iter()
            .map(|&n| n * glib_get_freq(freq_table, n))
            .sum();
        g_hash_table_destroy(freq_table);
        Ok(result)
    }
}
```

<!-- pause -->

<!-- new_line -->

**Key FFI concepts:**
* Manual memory management (create/destroy)
* Integers stored as void pointers (`gpointer`)
* Direct casting: `value as gpointer`
* Entire function body is `unsafe`
* Optional feature flag for system dependencies

<!-- end_slide -->



<!-- font_size: 7 -->

uthash: Macro-Based C Hash Table
===

<!-- speaker_note: |

    One more: uthash. Header-only, macro-based C library.

    Can't call macros from Rust FFI - need a C wrapper.

    [next: C code]

    HASH_FIND_INT, HASH_ADD_INT - these expand at compile time.

    We wrap them in real functions, compile via cc crate in build.rs.

    Now let's benchmark all these approaches.


    [20s]

 -->

<!-- new_line -->

<!-- font_size: 2 -->

## **Header-only C library using macros**

A popular C hash table that uses macros instead of functions.
We write a C wrapper and compile it with the `cc` crate.

<!-- pause -->

<!-- new_lines: 3 -->

### C Wrapper (uthash_wrapper.c)

```c
#include "uthash_wrapper.h"

struct hash_entry {
    int32_t key;        // the number
    int32_t count;      // frequency
    UT_hash_handle hh;  // uthash metadata
};

struct hash_entry* uthash_build_frequency_map(const int32_t* arr, size_t len) {
    struct hash_entry *table = NULL;
    for (size_t i = 0; i < len; i++) {
        struct hash_entry *entry;
        HASH_FIND_INT(table, &arr[i], entry);  // Macro lookup

        if (entry) {
            entry->count++;
        } else {
            entry = malloc(sizeof(struct hash_entry));
            entry->key = arr[i];
            entry->count = 1;
            HASH_ADD_INT(table, key, entry);    // Macro insert
        }
    }
    return table;
}
```

<!-- end_slide -->



<!-- font_size: 7 -->

uthash: Macro-Based C Hash Table
===

<!-- new_line -->

<!-- font_size: 2 -->

### Rust Side (extern "C" + cc crate)

<!-- new_line -->

```rust
unsafe extern "C" {
    fn uthash_build_frequency_map(arr: *const i32, len: usize) -> *mut HashEntry;
    fn uthash_lookup(hash_table: *mut HashEntry, key: i32) -> i32;
    fn uthash_destroy(hash_table: *mut HashEntry);
}

fn process_part_2_uthash(input: &str) -> Result<i32, String> {
    let (left, right) = unzip(input);

    unsafe {
        let table = uthash_build_frequency_map(right.as_ptr(), right.len());
        let result = left.iter()
            .map(|&n| n * uthash_lookup(table, n))
            .sum();
        uthash_destroy(table);
        Ok(result)
    }
}
```

<!-- new_lines: 4 -->

#### **Key differences from GLib:**

<!-- new_line -->

* Header-only (no runtime dependency)
* Macro-based API (not callable from Rust directly)
* Requires C wrapper functions
* Compiled via `cc` crate in build.rs

<!-- speaker_note: |

    and here's the rust code.

    now, on to the benchmarks!


    [??s]

 -->

<!-- end_slide -->

<!-- font_size: 7 -->

Benchmarks: At Scale
===

<!-- speaker_note: |

    now with the real puzzle input:

    part 1: no surprise here, rust is faster

    [next]

    part 2: at scale hashmap beats the naive implementation (as it should)

    but oops, our toy example is so contrived that FFI beats Rust. 
        Can we fix this? 


    [??s]

 -->

<!-- new_line -->

<!-- font_size: 2 -->

### Real Input — 1000 lines (criterion)

<!-- new_line -->

#### Part 1: Sorting

```
process rust sort       time:   [39.414 µs  39.554 µs  39.701 µs]

process c qsort         time:   [68.126 µs  68.518 µs  68.917 µs]
```

Rust still ~73% faster at scale.

<!-- pause -->

<!-- new_lines: 2 -->

#### Part 2: Frequency Counting

```
part 2 uthash           time:   [50.500 µs  50.670 µs  50.852 µs]

part 2 glib             time:   [50.988 µs  51.239 µs  51.527 µs]

part 2 hashmap          time:   [59.067 µs  59.269 µs  59.478 µs]

part 2 naive            time:   [77.219 µs  77.426 µs  77.664 µs]
```

**At scale, O(n) HashMap wins** — 30% faster than naive O(n*m)

<!-- pause -->


<!-- end_slide -->

<!-- font_size: 7 -->



<!-- font_size: 7 -->

ahash to the Rescue
===

<!-- speaker_note: |

    Drop in ahash - same API, faster hashing.

    [next]

    Rust wins! 9% faster than FFI.

    But notice - uthash and glib are close. FFI overhead is small here.

    This is exploratory - not production advice to use C hashmaps!

    Now let's flip it - call Rust FROM other languages.


    [20s]

 -->

<!-- new_line -->

<!-- font_size: 2 -->

### Drop-in replacement: `AHashMap` instead of `HashMap`

<!-- new_line -->

```rust
use ahash::AHashMap;

let counts: AHashMap<i32, usize> = right.iter()
    .fold(AHashMap::new(), |mut acc, &n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });
```

<!-- pause -->

<!-- new_line -->

### Real Input (1000 lines)

<!-- new_line -->

```
part 2 ahash            time:   [44.199 µs  44.347 µs  44.500 µs]

part 2 uthash           time:   [47.947 µs  48.217 µs  48.481 µs]

part 2 glib             time:   [48.987 µs  49.122 µs  49.263 µs]

part 2 hashmap          time:   [56.771 µs  56.998 µs  57.234 µs]

part 2 naive            time:   [75.450 µs  75.738 µs  76.043 µs]
```

<!-- pause -->

<!-- new_lines: 2 -->

**Rust reclaims the crown!** ahash is ~9% faster than uthash and glib.

<!-- new_line -->

* Swapping the hasher is all it took — same API, zero `unsafe`
* uthash and glib add almost negligible FFI overhead (~48 µs vs 44 µs)
* std HashMap's SipHash is DoS-resistant but ~29% slower than ahash
* The real bottleneck was the hash function, not the language

<!-- end_slide -->

<!-- font_size: 7 -->

UniFFI: From Rust to Every Language
===

<!-- speaker_note: |

    Everything so far: Rust calling C.

    UniFFI flips it: other languages calling Rust.

    Mozilla built this for Firefox - Rust core, expose everywhere.

    [next: setup]

    Need cdylib crate type. Edition 2021 required (gotcha).

    [next: UDL]

    UDL declares interface. Build script generates scaffolding.

    Three lines, four languages: Python, Kotlin, Swift, Ruby.


    [25s]

-->

<!-- new_line -->

<!-- font_size: 2 -->

## **Mozilla's UniFFI generates bindings for Python, Kotlin, Swift, Ruby**

All our FFI so far: Rust **calling** C/C++.
UniFFI flips it: other languages **calling** Rust.

### Setup

<!-- new_line -->

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]  # Shared library for FFI
name = "aoc_ffi_day01"

[dependencies]
uniffi = "0.28"
thiserror = "1.0"  # Required for UniFFI error types

[build-dependencies]
uniffi = { version = "0.28", features = ["build"] }
```

**Caveat:** UniFFI 0.28 requires edition 2021 (not 2024)

<!-- pause -->

<!-- new_line -->

### UDL + Proc Macros

```rust
// src/aoc_ffi_day01.udl — minimal, functions defined via macros
namespace aoc_ffi_day01 {};
```

```rust
// build.rs — generate scaffolding from UDL
uniffi::generate_scaffolding("src/aoc_ffi_day01.udl")?;
```

```rust
// lib.rs — include generated code
uniffi::include_scaffolding!("aoc_ffi_day01");
```

<!-- end_slide -->

<!-- font_size: 7 -->

UniFFI: From Rust to Every Language
===

<!-- speaker_note: |

    UniFFI constraints: no references across FFI.

    Need owned types - String not &str.

    [next: code]

    Thin wrappers: take String, call existing impl, convert errors.

    thiserror for ergonomics, uniffi::Error for the macro.

    Pattern: wrapper delegates, don't rewrite logic.


    [20s]

-->

<!-- new_line -->

<!-- font_size: 2 -->

### Exporting Functions

<!-- new_line -->

```rust
// UniFFI needs owned types (String, not &str) and its own error type
#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum AocError {
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[uniffi::export]
pub fn uniffi_process_rust_sort(input: String) -> Result<i32, AocError> {
    process_rust_sort(&input).map_err(Into::into)
}
```

<!-- new_line -->

**Key constraints:**
* No `&str` — UniFFI needs owned `String` across FFI boundary
* Custom error type with `thiserror` + `uniffi::Error`
* Thin wrappers delegate to existing implementations

<!-- end_slide -->



<!-- font_size: 7 -->

Python Bindings via UniFFI
===

<!-- speaker_note: |

    Two commands: build library, generate bindings.

    [next: command]

    uniffi-bindgen reads UDL, generates Python module with ctypes.

    [next: Python]

    Just import and call. All implementations exposed identically.

    Same Rust code. Zero Python-specific logic.


    [15s]
-->

<!-- new_line -->

<!-- font_size: 2 -->

## **Generate and call Rust from Python**

<!-- new_line -->

### Generate bindings

<!-- new_line -->

```bash
# Build the shared library
cargo build --release --lib

# Generate Python module from UDL + compiled library
uniffi-bindgen generate src/aoc_ffi_day01.udl \
    --library target/release/libaoc_ffi_day01.so \
    --language python \
    --out-dir bindings/python
```

<!-- new_line -->

UniFFI produces a `.py` module that loads the `.so` via `ctypes`.

<!-- pause -->

<!-- new_line -->

### Calling Rust from Python

<!-- new_line -->

```python
import aoc_ffi_day01

sample = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3"

# Part 1 — all three sorting backends
rust_sort = aoc_ffi_day01.uniffi_process_rust_sort(sample)   # 11
c_qsort   = aoc_ffi_day01.uniffi_process_c_qsort(sample)     # 11
cpp_sort   = aoc_ffi_day01.uniffi_process_cpp_sort(sample)    # 11

# Part 2 — hash table backends
naive  = aoc_ffi_day01.uniffi_process_part_2(sample)          # 31
hashmap = aoc_ffi_day01.uniffi_process_part_2_hashmap(sample) # 31
uthash  = aoc_ffi_day01.uniffi_process_part_2_uthash(sample)  # 31
```

<!-- end_slide -->



<!-- font_size: 7 -->

Python Bindings via UniFFI
===

<!-- speaker_note: |

    Errors just work - Rust Result becomes Python exception.

    [next: try/except]

    AocError in Rust = AocError in Python.

    One library, one UDL, four languages.

    No hand-written bindings. One command per language.

    Kotlin next.


    [15s]

-->

<!-- new_line -->

<!-- font_size: 2 -->

### Error handling crosses the FFI boundary

<!-- new_line -->

```python
try:
    result = aoc_ffi_day01.uniffi_process_rust_sort(bad_input)
except aoc_ffi_day01.AocError as e:
    print(f"Rust error caught in Python: {e}")
```

<!-- new_line -->

**What UniFFI gives us:**
* Python module auto-generated from Rust types
* Rust `Result<T, E>` → Python exceptions
* No hand-written C bindings or `ctypes` declarations
* **Also generates Kotlin (JNA), Swift, and Ruby** — same shared library, one command each

<!-- end_slide -->

<!-- font_size: 7 -->

Kotlin Bindings via UniFFI
===

<!-- speaker_note: |

    Same library, JVM target via Kotlin.

    Same command, different flag.

    [next: command]

    Generates Kotlin with JNA - different from Python's ctypes, same .so.

    [next: Kotlin code]

    Import and call. Identical to Python.

    Now works on Android, server-side, anywhere JVM runs.


    [15s]

-->

<!-- new_line -->

<!-- font_size: 2 -->

## **Same Rust library, now from the JVM**

<!-- new_line -->

### Generate bindings

<!-- new_line -->

```bash
uniffi-bindgen generate src/aoc_ffi_day01.udl \
    --library target/release/libaoc_ffi_day01.so \
    --language kotlin \
    --out-dir bindings/kotlin
```

UniFFI produces a `.kt` file that uses **JNA** to load the shared library.

<!-- pause -->

<!-- new_line -->

### Calling Rust from Kotlin

<!-- new_line -->

```kotlin
import uniffi.aoc_ffi_day01.uniffiProcessRustSort
import uniffi.aoc_ffi_day01.uniffiProcessPart2Hashmap
import uniffi.aoc_ffi_day01.AocError

val sample = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3"

val rustSort = uniffiProcessRustSort(sample)   // 11
val hashmap  = uniffiProcessPart2Hashmap(sample) // 31
```

<!-- end_slide -->

<!-- font_size: 7 -->

Kotlin Bindings via UniFFI
===

<!-- speaker_note: |

    UniFFI adapts to each language's conventions.

    Snake case → camelCase automatically.

    [next: error handling]

    Result → exceptions, just like Python.

    Python uses ctypes, Kotlin uses JNA.

    We wrote Rust once. UniFFI handles everything.

    Swift... was harder.


    [15s]

-->

<!-- new_line -->

<!-- font_size: 2 -->

### Kotlin-specific details

**UniFFI transforms names to Kotlin conventions:**
* `uniffi_process_rust_sort` → `uniffiProcessRustSort` (camelCase)
* `uniffi_process_part_2` → `uniffiProcessPart2`

**Error handling maps to exceptions:**
```kotlin
try {
    val result = uniffiProcessRustSort(input)
} catch (e: AocError) {
    println("Rust error caught in Kotlin: ${e.message}")
}
```

**Python uses `ctypes`, Kotlin uses JNA** — different FFI
mechanisms, same generated shared library.

<!-- end_slide -->

<!-- font_size: 7 -->

Swift Bindings via UniFFI
===

<!-- speaker_note: |

    Swift was harder. Even on macOS.

    [next: command]

    Generates THREE files: Swift code, C header, modulemap.

    Goes through Objective-C interop, not ctypes/JNA.

    [next: Swift code]

    API looks clean - named params, do/catch.

    [next: compile]

    But compilation is painful.

    Manual linking, header imports, library paths.

    Python/Kotlin: just import. Swift: archaeology.

    On Linux with Nix? glibc conflicts. Containerized it.


    [25s]

-->

<!-- font_size: 2 -->

## **Same Rust library, now from Swift**

### Generate bindings

```bash
uniffi-bindgen generate src/aoc_ffi_day01.udl \
    --library target/release/libaoc_ffi_day01.so \
    --language swift \
    --out-dir bindings/swift
```

UniFFI produces **three files:**
* `aoc_ffi_day01.swift` — Swift wrapper code
* `aoc_ffi_day01FFI.h` — C header for the FFI layer
* `aoc_ffi_day01FFI.modulemap` — Clang module map

<!-- pause -->

### Calling Rust from Swift

```swift
do {
    let rustSort = try uniffiProcessRustSort(input: SAMPLE_INPUT)
    let cQsort   = try uniffiProcessCQsort(input: SAMPLE_INPUT)
    let hashmap  = try uniffiProcessPart2Hashmap(input: SAMPLE_INPUT)
} catch let error as AocException {
    print("Rust error caught in Swift: \(error)")
}
```

<!-- pause -->

### Compile and link

```bash
swiftc -o test_swift test_swift_bindings.swift aoc_ffi_day01.swift \
    -import-objc-header aoc_ffi_day01FFI.h \
    -L . -laoc_ffi_day01
LD_LIBRARY_PATH=. ./test_swift
```

**Swift-specific details:**
* Named parameters: `uniffiProcessRustSort(input: ...)` not just `(…)`
* Errors use `do/catch` with typed `AocException`
* Needs C header + modulemap (vs Python's `ctypes` / Kotlin's JNA)

<!-- end_slide -->


<!-- font_size: 7 -->

Lessons Learned
===

<!-- speaker_note: |

    What I learned:

    [next: cross-platform]

    Platform differences are real. .so, .dylib, .dll.

    Automated with just - detect at build time.

    [next: tooling]

    Each language has quirks. Python needs venv --copies in Nix.

    Kotlin's JNA doesn't search current dir.

    Don't assume - inspect generated bindings.

    [next: automation]

    Automation saved me. just + Nix + documentation.

    CLAUDE.md because I'll forget the glibc fix.

    [next: performance]

    Rust usually wins. Hash choice matters more than language.

    Big-O lies - benchmark YOUR data.

    [next: FFI]

    FFI is pragmatic bridging, not dogma.

    Choose tools for context, not ideology.


    [40s]

-->

<!-- font_size: 2 -->

<!-- pause -->

<!-- incremental_lists: true -->

### 1. Cross-platform FFI requires platform awareness
* Library extensions differ: `.so` / `.dylib` / `.dll`
* Build artifacts isolation needed (Docker volumes for devcontainers)
* Dynamic detection at build time via `just` — not runtime

<!-- new_line -->

### 2. Tooling integration is complex
* **Python:** Nix's immutable `/nix/store` blocks pip — requires `venv --copies`
* **Kotlin:** JNA doesn't search current directory — needs `-J-Djna.library.path=.`
* **UniFFI:** Generated APIs don't match assumptions — always inspect bindings

<!-- new_line -->

### 3. Automation saves sanity
* `just` for task orchestration across languages
* `Nix` for reproducible environments
* Document everything in `CLAUDE.md`

<!-- new_line -->

### 4. Performance surprises
* Pure Rust usually wins (monomorphization, inlining)
* Hash function choice matters more than language (ahash > SipHash)
* **Always benchmark YOUR data**

<!-- new_line -->

### 5. FFI makes you resilient
* Pragmatic bridge for legacy integration
* Choose tools based on context, not dogma

<!-- incremental_lists: false -->

<!-- end_slide -->



<!-- font_size: 7 -->

What's Next?
===

<!-- speaker_note: |

    [next: open source]

    Contributing back - LearnXinY PR for Dart/Rust FFI docs.

    Documenting what I wish existed.

    [next: ongoing work]

    Swift in devcontainers - still fighting glibc.

    Docker-based testing workaround.

    Evaluating Ditto's safer_ffi - macros, no UDL, auto C headers.

    [next: upcoming talks]

    Next: CRDTs with Ditto. Distributed AoC solving.

    Learning in public - AoC lessons already inform Ditto's devcontainers.

    Nothing wasted when you document the messy middle.


    [30s]

-->

<!-- font_size: 4 -->

<!-- new_line -->

<!-- incremental_lists: true -->

## Open Source Contributions

- **LearnXinY**: Filling documentation gaps for Dart/Rust FFI
    + [Draft PR](https://github.com/adambard/learnxinyminutes-docs/pulls/alycda)

<!-- new_line -->

## Ongoing Work

- **Swift in DevContainers**: Solving the glibc conflict ([PR #5](https://github.com/alycda/aoc-ffi/pull/5))
    + Docker-based testing to work around Nix glibc incompatibility
- evaluating Ditto's [safer_ffi](https://github.com/getditto/safer_ffi)
    + Macro-based FFI without UDL, auto-generates C headers

<!-- new_line -->

## Upcoming Talks

- **Ditto's CRDTs**: Distributed AoC solving 
    + Stay tuned for the next chapter of my learning journey!

<!-- incremental_lists: false -->

<!-- speaker_note: |

    The glibc lessons from this project directly informed the devcontainer approach. And I'm excited to apply these FFI learnings to Ditto's CRDT work - exploring safer_ffi as an alternative to UniFFI for production use cases. 


    [??s]
    -->

<!-- end_slide -->


<!-- font_size: 7 -->

Questions?
===

<!-- new_line -->

![image:w:75%](./presentation/img/rustacean-flat-gesture.png)

<!-- speaker_note: ... -->

<!-- end_slide -->

<!-- skip_slide -->

<!-- font_size: 7 -->

Q: Did I use AI?
===

<!-- font_size: 3 -->

<!-- new_line -->

**Yep! Claude Code extensively.**

<!-- new_line -->

<!-- incremental_lists: true -->
* Helped debug cryptic FFI errors (JNA paths, Nix venv issues)
* Generated boilerplate bindings and test scaffolding
* Pair programming on build.rs and justfile recipes
* Caught the integer overflow bug in compare_i32
* Documented lessons learned in CLAUDE.md

<!-- incremental_lists: false -->

<!-- pause -->

<!-- new_line -->

**Key insight:** AI excels at FFI because it's seen thousands of Stack Overflow posts about these exact tooling quirks.

<!-- new_line -->

But you still need to understand what you're asking for and verify the results.

<!-- end_slide -->

<!-- skip_slide -->

<!-- font_size: 7 -->

Q: Why do I hate LeetCode/HackerRank?
===

<!-- font_size: 3 -->

<!-- new_line -->

**They optimize for interview performance, not learning.**

<!-- new_line -->

<!-- incremental_lists: true -->
* **Arbitrary constraints** — "solve in O(n log n)" without context
* **No real-world engineering** — no tests, no docs, no refactoring
* **Gatekeeping culture** — memorize patterns or fail interviews
* **notoriously bad support for rust** - limited compiler output, reading the docs is considered "cheating"
* **Disconnected from actual work** — when's the last time you implemented sort manually? Oh wait...

<!-- incremental_lists: false -->

<!-- pause -->

<!-- new_line -->

**Advent of Code is different:**
* **Real problems** — parsing, state machines, graph traversal
* **No time pressure** — solve at your own pace
* **Community-driven** — learn from others' solutions
* **Playground mentality** — try different approaches, benchmark, experiment

<!-- new_line -->

AoC rewards curiosity. LeetCode rewards pattern memorization.

<!-- end_slide -->

<!-- skip_slide -->

<!-- font_size: 7 -->

Q: What's Ditto?
===

<!-- font_size: 3 -->

<!-- new_line -->

P2P cloud-optional sync

<!-- new_line -->
Ditto solves one of the hardest problems in distributed systems: seamless data sync at the edge, regardless of network conditions.

<!-- new_line -->

<!-- incremental_lists: true -->
* Devices sync directly P2P (Bluetooth, WiFi Direct, local network)
* Works offline — cloud is optional, not required
* CRDTs ensure eventual consistency without conflicts
* Used in healthcare, military, logistics — anywhere connectivity is unreliable

<!-- incremental_lists: false -->

<!-- pause -->

<!-- new_line -->

**Why it's relevant to this talk:**
* Heavy use of FFI — Rust core with Swift/Kotlin/C++ SDKs
* Uses `safer_ffi` for production FFI (not UniFFI)
* Real-world constraints inform my FFI learning

<!-- end_slide -->

<!-- skip_slide -->

<!-- font_size: 7 -->

Q: Do I have a CS degree?
===

<!-- font_size: 3 -->

<!-- new_line -->

**Nope!**

<!-- new_line -->

I am self-taught with a **BFA in Graphic Design** and almost a minor in Photography.

<!-- new_line -->

<!-- incremental_lists: true -->
* I love Typography and Color Theory
* I also wanted to pursue Motion Graphics
* 20 years of breaking things on the internet
* Learning in public because traditional CS education isn't the only path

<!-- incremental_lists: false -->

<!-- pause -->

<!-- new_line -->

**Design background helps:**
* Attention to detail (crucial for FFI pointer safety)
* Systems thinking (how components interact)
* Iterative refinement (benchmark, optimize, repeat)

<!-- new_line -->

You don't need a CS degree to understand `unsafe extern "C"`.

You just need curiosity and willingness to read compiler errors.
<!-- end_slide -->


<!-- font_size: 7 -->

Thank You!
===

<!-- font_size: 4 -->

**Alyssa Evans**
<!-- new_line -->
Staff Software Engineer @ [Ditto](https://ditto.com)


## Socials
- 🐙 github.com/alycda
- 💼 linkedin.com/in/alyda

![](./presentation/img/LinkedIn.jpeg)


### Resources:
- 🎄 [AdventOfCode](https://adventofcode.com)
- 📝 Learn X in Y Minutes PRs: https://github.com/adambard/learnxinyminutes-docs/pulls/alycda
- 🎞️ This presentation: https://github.com/alycda/aoc-ffi/blob/main/slides.md


<!-- speaker_note: |
    Thanks again for having me. If I didn't get to your question, or if you think of something later, please reach out. I'm always happy to talk about Rust, FFI, or why I race motorcycles and other extreme sports.

    Enjoy the rest of the meetup!


    [??s]
 -->

<!-- end_slide -->

<!-- font_size: 7 -->

Errata
===

<!-- new_line -->

<!-- font_size: 5 -->

> "No piece of writing is ever finished. It’s just due."

<!-- new_line -->
--- Bill Condon

<!-- new_line -->

<!-- font_size: 3 -->

<!-- pause -->

## Tools Used

<!-- incremental_lists: false -->
- 🎁 [**Presenterm**](https://mfontanini.github.io/presenterm/) — terminal-based slideshow ([contributed PR #826](https://github.com/mfontanini/presenterm/pull/826))
- 🦀 [**Ferris**](https://rustacean.net/) — unofficial Rust mascot
- ❄️ [**Nix**](https://NixOS.org) — reproducible dev environments
- ⚔️ [**Jujutsu**](https://www.jj-vcs.dev/) — next-gen VCS (Git-compatible)
- 🐽 [**Bacon**](https://dystroy.org/bacon/) — background Rust code checker
- ☑️ [**Workflowy**](https://workflowy.com) — outlining and organizing thoughts
- 🤖 [**Claude Code**](https://claude.ai/claude-code) — AI pair programming
<!-- incremental_lists: false -->

<!-- new_line -->

### This Talk

📦 **Code & Slides**: [github.com/alycda/aoc-ffi](https://github.com/alycda/aoc-ffi)

<!-- speaker_note:
A quick note on tools: I even contributed a PR to Presenterm while preparing this talk - I added a feature, and now you're witnessing the result. That's learning in public in action.

All the code, benchmarks, and these slides are on GitHub. Feel free to clone, experiment, and break things yourself - that's how we learn.


    [??s]
-->

<!-- no_footer -->

<!-- end_slide -->

<!-- font_size: 7 -->

Zero-Cost Abstractions
===

<!-- font_size: 2 -->

<!-- new_line -->

### Generic Solve

<!-- new_line -->

```rust
pub fn solve<S: Sorter>(input: &str) -> Result<i32, String> {
    let (mut left, mut right) = unzip(input);
    S::sort(&mut left);
    S::sort(&mut right);
    Ok(left.iter().zip(right.iter())
        .map(|(l, r)| (l-r).abs()).sum::<i32>())
}
```

<!-- pause -->

<!-- new_lines: 2 -->

### Convenience Wrappers

<!-- new_line -->

```rust
pub fn process_c_qsort(input: &str) -> Result<i32, String> {
    solve::<CSort>(input)
}

pub fn process_rust_sort(input: &str) -> Result<i32, String> {
    solve::<UnstableSort>(input)
}

pub fn process_rust_sort_stable(input: &str) -> Result<i32, String> {
    solve::<StableSort>(input)
}
```

<!-- pause -->

<!-- new_lines: 2 -->

### Why is this zero-cost?

<!-- new_line -->

<!-- incremental_lists: true -->
* `StableSort`, `UnstableSort`, and `CSort` are **Zero-Sized Types** (no runtime memory)
* Compiler **monomorphizes** `solve<S>` into **three** specialized functions
* No vtables, no dynamic dispatch — all resolved **at compile time**
* Same as C++ templates, but with trait bounds instead of SFINAE
<!-- incremental_lists: false -->