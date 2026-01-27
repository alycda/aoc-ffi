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

Lessons Learned
===

<!-- font_size: 2 -->

<!-- pause -->

## **Cross-platform UniFFI is hard**

<!-- new_line -->

### The Problem

UniFFI generates bindings that reference **platform-specific** libraries:

<!-- new_line -->

- macOS: `.dylib`
- Linux: `.so`
- Windows: `.dll`

<!-- pause -->

<!-- new_line -->

### The Solution

```justfile
# Determine library extension based on platform
lib_ext := if os() == "macos" { "dylib" }
           else if os() == "windows" { "dll" }
           else { "so" }

uniffi-gen-python: build-lib
    uniffi-bindgen generate src/aoc_ffi_day01.udl \
        --lib-file target/release/libaoc_ffi_day01.{{lib_ext}} \
        --language python
    cp target/release/libaoc_ffi_day01.{{lib_ext}} bindings/python/
```

<!-- new_line -->

**Key insight:** Dynamic detection at build time, not runtime

<!-- end_slide -->

<!-- font_size: 7 -->

Nix + Python = 💔
===

<!-- font_size: 2 -->

<!-- pause -->

### The Error

```
error: externally-managed-environment
× This environment is externally managed
╰─> This command has been disabled as it tries to modify
    the immutable `/nix/store` filesystem.
```

<!-- new_line -->

**Problem:** Nix's immutable `/nix/store` prevents direct pip installations

<!-- pause -->

<!-- new_line -->

### The Fix

```nix
shellHook = ''
  # Create isolated venv
  if [ ! -d ".venv" ]; then
    python3 -m venv .venv --copies  # ← --copies is crucial
  fi

  # Verify activation before installing
  if [ -f ".venv/bin/activate" ]; then
    source .venv/bin/activate
  else
    # Handle incomplete venv
    rm -rf .venv
    python3 -m venv .venv --copies
    source .venv/bin/activate
  fi

  # Now pip works!
  pip install uniffi-bindgen==0.28.3
''
```

<!-- end_slide -->

<!-- font_size: 7 -->

JNA: "Where's My Library?"
===

<!-- font_size: 2 -->

<!-- pause -->

### The Error

```
java.lang.UnsatisfiedLinkError: Unable to load library 'aoc_ffi_day01'
libaoc_ffi_day01.so: cannot open shared object file: No such file or directory
Native library (linux-aarch64/libaoc_ffi_day01.so) not found in resource path
```

<!-- new_line -->

**Problem:** JNA doesn't look in current directory by default

<!-- pause -->

<!-- new_line -->

### The Fix

```justfile
uniffi-test-kotlin: compile-kotlin
    cd bindings/kotlin && \
    kotlinc -J-Djna.library.path=. \  # ← Tell JNA where to look
            -script ../../tests/kotlin/test_kotlin_bindings.kts \
            -classpath "aoc_ffi_day01.jar:$HOME/.m2/repository/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar"
```

<!-- new_line -->

**Key points:**
* `-J-D` passes JVM system properties through kotlinc
* `jna.library.path=.` adds current directory to search path
* JNA JAR must be on classpath for both compile **and** runtime

<!-- end_slide -->

<!-- font_size: 7 -->

DevContainer Isolation
===

<!-- font_size: 2 -->

<!-- pause -->

### The Problem

<!-- new_line -->

Sharing `.venv/` and `target/` between host (macOS) and container (Linux):

<!-- incremental_lists: true -->
* Python packages built for macOS don't work in Linux
* Rust build artifacts are architecture-specific
* Library files have wrong extensions (`.dylib` vs `.so`)
* Constantly rebuilding when switching environments
<!-- incremental_lists: false -->

<!-- pause -->

<!-- new_line -->

### The Solution

Use **Docker volumes** to isolate per-platform artifacts:

```json
{
  "mounts": [
    "source=aoc-ffi-venv,target=${containerWorkspaceFolder}/.venv,type=volume",
    "source=aoc-ffi-target,target=${containerWorkspaceFolder}/aoc-2024-12-01/target,type=volume"
  ]
}
```

<!-- pause -->

<!-- new_line -->

**Result:**
* macOS `.venv` and `target/` → local filesystem
* Linux `.venv` and `target/` → Docker volumes
* Zero conflicts, faster rebuilds, cached per platform

<!-- end_slide -->

<!-- font_size: 7 -->

UniFFI API Surprises
===

<!-- font_size: 2 -->

<!-- pause -->

### The Problem

<!-- new_line -->

Generated Kotlin code didn't match our expectations:

<!-- incremental_lists: true -->
* Test imported `uniffiProcessCppSort` → **doesn't exist** (only had C qsort)
* Test caught `AocError` → **wrong name** (generated as `AocException`)
* Function names transformed: `uniffi_process_rust_sort` → `uniffiProcessRustSort`
<!-- incremental_lists: false -->

<!-- pause -->

<!-- new_line -->

### The Lesson

**Always inspect generated bindings before writing tests**

```bash
# Check what UniFFI actually generated
jar tf bindings/kotlin/aoc_ffi_day01.jar | grep -i "process\|aocerror"

# Or read the generated .kt file
grep "^fun uniffi\|^class Aoc" bindings/kotlin/uniffi/aoc_ffi_day01/aoc_ffi_day01.kt
```

<!-- new_line -->

**Key takeaway:** Generated code is the source of truth, not your assumptions

<!-- end_slide -->

<!-- font_size: 7 -->

glibc "Cross-Compilation"
===

<!-- font_size: 2 -->

<!-- pause -->

### The Problem

<!-- new_line -->

Nix + Swift = symbol lookup error:

```
./test_swift: symbol lookup error:
  /nix/store/.../libc.so.6: undefined symbol:
  __tunable_is_initialized, version GLIBC_PRIVATE
```

<!-- pause -->

<!-- new_line -->

**Root cause:** Mixing different glibc versions is like cross-compiling

<!-- incremental_lists: true -->
* Nix's Rust uses glibc 2.42 from `/nix/store`
* System Swift uses glibc 2.39 from `/lib`
* Rust `.so` built with Nix glibc can't be loaded by system Swift
* Can't install both tools with matching glibc (home-manager vs system PATH)
<!-- incremental_lists: false -->

<!-- pause -->

<!-- new_line -->

### The Solution

**Docker isolation** — Build once, test in matching environment:

```justfile
uniffi-test-swift-docker: uniffi-gen-swift
    docker run --rm \
        -v "$(pwd)/aoc-2024-12-01:/workspace" \
        -w /workspace/bindings/swift \
        swift:bookworm \
        bash -c "swiftc -o test_swift ..."
```

<!-- new_line -->

**Key insight:** Use containers to separate incompatible glibc environments,
just like you'd use cross-compilation for different architectures.

<!-- end_slide -->

<!-- font_size: 7 -->

Build System Complexity
===

<!-- font_size: 2 -->

<!-- pause -->

### The Challenge

<!-- new_line -->

Multiple languages × multiple platforms × multiple tools:

```
┌─────────────┬──────────────┬─────────────────────┐
│  Language   │  Build Tool  │  FFI Mechanism      │
├─────────────┼──────────────┼─────────────────────┤
│  Rust       │  cargo       │  cdylib             │
│  Python     │  pip/venv    │  ctypes             │
│  Kotlin     │  kotlinc     │  JNA                │
│  Swift      │  Docker      │  glibc isolation    │
│  C          │  cc crate    │  extern "C"         │
│  All        │  just        │  task orchestration │
│  All        │  Nix         │  environment        │
└─────────────┴──────────────┴─────────────────────┘
```

<!-- pause -->

<!-- new_line -->

### The Solution

**Centralize everything in `justfile`:**

```justfile
# One command to rule them all
uniffi-test: uniffi-test-python uniffi-test-kotlin uniffi-test-swift

# Swift auto-falls back to Docker on Linux
uniffi-test-swift: uniffi-gen-swift
    @if ! command -v swiftc &> /dev/null; then
        just uniffi-test-swift-docker
    fi

# Clean slate when switching platforms
clean-bindings:
    rm -rf aoc-2024-12-01/bindings/**/*.{py,dylib,so,dll,jar}
```

<!-- new_line -->

**Document the workflow in `CLAUDE.md`** so future you (and Claude) understand the setup

<!-- end_slide -->

<!-- font_size: 7 -->

Key Takeaways
===

<!-- font_size: 2 -->

<!-- pause -->

<!-- incremental_lists: true -->

1. **Isolate per platform** — Don't share build artifacts between macOS and Linux

2. **Automate everything** — Manual steps lead to errors and confusion

3. **Document thoroughly** — `CLAUDE.md` saved us multiple times

4. **Test on all targets** — What works on macOS may fail on Linux

5. **Use declarative tools** — Nix ensures reproducible environments

6. **Gitignore generated files** — Commit sources, not outputs

7. **Inspect generated code** — Don't assume APIs, verify them

8. **glibc = architecture** — Treat different libc versions like different platforms

<!-- incremental_lists: false -->

<!-- end_slide -->

<!-- font_size: 7 -->

What Worked Well
===

<!-- font_size: 2 -->

<!-- pause -->

<!-- new_line -->

✅ **Nix** — Reproducible dev environments across platforms

✅ **UniFFI** — Zero-boilerplate FFI to Python, Kotlin, Swift

✅ **Just** — Simple, powerful task runner (better than Make)

✅ **Docker volumes** — Clean isolation for devcontainers

✅ **Docker-in-Docker** — Run Swift tests in isolated glibc environment

✅ **Direnv** — Automatic environment activation per directory

✅ **Rust** — Fast, safe core with battle-tested FFI

<!-- end_slide -->

<!-- font_size: 7 -->

Final Thoughts
===

<!-- font_size: 2 -->

<!-- pause -->

<!-- new_line -->

## Cross-platform FFI development requires:

<!-- new_line -->

<!-- incremental_lists: true -->
* Patience with tooling quirks (looking at you, JNA)
* Deep understanding of each platform's conventions
* Robust automation to catch platform-specific issues
* Clear documentation for context switching
* Willingness to debug cryptic errors at 3 AM
<!-- incremental_lists: false -->

<!-- pause -->

<!-- new_line -->

<!-- new_line -->

### **The juice is worth the squeeze**

<!-- new_line -->

When you need native performance across multiple languages and platforms,
FFI is still the best tool for the job.

<!-- new_line -->

**Just be ready for the journey.** 🚀

<!-- end_slide -->