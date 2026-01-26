---
theme:
    name: catppuccin-mocha
    override:
        footer:
            style: template
            left: "Learning in Public : AOC FFI Playground"
            center: github.com/alycda/learning-in-public/tree/aoc-ffi
---

<!-- font_size: 7 -->

Questions?
===

<!-- new_line -->

![image:w:75%](./img/rustacean-flat-gesture.png)

<!-- speaker_note: ... -->

<!-- end_slide -->

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
