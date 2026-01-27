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

What's Next?
===

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

<!-- speaker_note: The glibc lessons from this project directly informed the devcontainer approach. And I'm excited to apply these FFI learnings to Ditto's CRDT work - exploring safer_ffi as an alternative to UniFFI for production use cases. -->

<!-- end_slide -->