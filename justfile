_default: 
    @just --list

rebuild:
    home-manager switch -b backup -f .devcontainer/home.nix

export USER := shell("whoami")

# Determine library extension based on platform
lib_ext := if os() == "macos" { "dylib" } else if os() == "windows" { "dll" } else { "so" }

cheats:
    cheat -l

present:
    presenterm slides.md

present-with-speaker-notes:
    tmux kill-session -t present 2>/dev/null || true
    tmux new-session -d -s present 'presenterm --listen-speaker-notes slides.md' \; \
        split-window -h 'presenterm --publish-speaker-notes slides.md' \; \
        attach -t present

[working-directory: 'aoc-2024-12-01']
aoc-test:
    cargo test

[working-directory: 'aoc-2024-12-01']
aoc-bench:
    cargo bench

[working-directory: 'aoc-2024-12-01']
aoc-run:
    cargo run

# UniFFI bindings - clean generated files
clean-bindings:
    @echo "Cleaning generated UniFFI bindings..."
    rm -rf aoc-2024-12-01/bindings/python/*.py
    rm -rf aoc-2024-12-01/bindings/python/*.dylib
    rm -rf aoc-2024-12-01/bindings/python/*.so
    rm -rf aoc-2024-12-01/bindings/python/*.dll
    rm -rf aoc-2024-12-01/bindings/kotlin/uniffi/
    rm -rf aoc-2024-12-01/bindings/kotlin/*.dylib
    rm -rf aoc-2024-12-01/bindings/kotlin/*.so
    rm -rf aoc-2024-12-01/bindings/kotlin/*.dll
    rm -rf aoc-2024-12-01/bindings/kotlin/*.jar
    rm -rf aoc-2024-12-01/bindings/swift/*.swift
    rm -rf aoc-2024-12-01/bindings/swift/*.h
    rm -rf aoc-2024-12-01/bindings/swift/*.modulemap
    rm -rf aoc-2024-12-01/bindings/swift/*.dylib
    rm -rf aoc-2024-12-01/bindings/swift/*.so
    rm -rf aoc-2024-12-01/bindings/swift/*.dll
    @echo "✓ Bindings cleaned"

# UniFFI bindings - build library
[working-directory: 'aoc-2024-12-01']
build-lib:
    cargo build --release --lib

# UniFFI Python bindings
uniffi-gen-python: build-lib
    mkdir -p aoc-2024-12-01/bindings/python
    uniffi-bindgen generate aoc-2024-12-01/src/aoc_ffi_day01.udl \
        --lib-file aoc-2024-12-01/target/release/libaoc_ffi_day01.{{lib_ext}} \
        --language python \
        --out-dir aoc-2024-12-01/bindings/python
    cp aoc-2024-12-01/target/release/libaoc_ffi_day01.{{lib_ext}} aoc-2024-12-01/bindings/python/
    @echo "✓ Python bindings generated in aoc-2024-12-01/bindings/python/"

uniffi-test-python: uniffi-gen-python
    cd aoc-2024-12-01 && PYTHONPATH=bindings/python python3 tests/python/test_python_bindings.py

# UniFFI Kotlin bindings
uniffi-gen-kotlin: build-lib
    mkdir -p aoc-2024-12-01/bindings/kotlin
    uniffi-bindgen generate aoc-2024-12-01/src/aoc_ffi_day01.udl \
        --lib-file aoc-2024-12-01/target/release/libaoc_ffi_day01.{{lib_ext}} \
        --language kotlin \
        --out-dir aoc-2024-12-01/bindings/kotlin
    cp aoc-2024-12-01/target/release/libaoc_ffi_day01.{{lib_ext}} aoc-2024-12-01/bindings/kotlin/
    @echo "✓ Kotlin bindings generated in aoc-2024-12-01/bindings/kotlin/"

compile-kotlin: uniffi-gen-kotlin
    @echo "Compiling Kotlin bindings..."
    cd aoc-2024-12-01/bindings/kotlin && kotlinc -classpath "$HOME/.m2/repository/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar" uniffi/aoc_ffi_day01/aoc_ffi_day01.kt -include-runtime -d aoc_ffi_day01.jar

uniffi-test-kotlin: compile-kotlin
    @echo "Testing Kotlin bindings..."
    cd aoc-2024-12-01/bindings/kotlin && kotlinc -J-Djna.library.path=. -script ../../tests/kotlin/test_kotlin_bindings.kts -classpath "aoc_ffi_day01.jar:$HOME/.m2/repository/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar"

# UniFFI Swift bindings
build-lib-no-glib:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building library without GLib (for Swift compatibility)..."
    # On Linux with Nix, use special script to build outside Nix environment
    if [ "$(uname)" = "Linux" ] && [ -n "${IN_NIX_SHELL:-}" ] && [ -f .devcontainer/build-swift-lib.sh ]; then
        bash .devcontainer/build-swift-lib.sh
    else
        cd aoc-2024-12-01 && cargo build --release --lib --no-default-features
    fi

uniffi-gen-swift: build-lib-no-glib
    mkdir -p aoc-2024-12-01/bindings/swift
    uniffi-bindgen generate aoc-2024-12-01/src/aoc_ffi_day01.udl \
        --lib-file aoc-2024-12-01/target/release/libaoc_ffi_day01.{{lib_ext}} \
        --language swift \
        --out-dir aoc-2024-12-01/bindings/swift
    cp aoc-2024-12-01/target/release/libaoc_ffi_day01.{{lib_ext}} aoc-2024-12-01/bindings/swift/
    @echo "✓ Swift bindings generated in aoc-2024-12-01/bindings/swift/"
    @echo "Note: GLib-based implementation (uniffiProcessPart2Glib) is unavailable"

uniffi-test-swift: uniffi-gen-swift
    @echo "Testing Swift bindings..."
    @if ! command -v swiftc &> /dev/null; then \
        echo "⚠️  Swift compiler not found. Please install Swift."; \
        exit 1; \
    fi
    @if [ -n "${IN_NIX_SHELL:-}" ] && [ "$(uname)" = "Linux" ]; then \
        echo "⚠️  Skipping Swift tests in Nix devcontainer due to glibc conflicts"; \
        echo "    Swift bindings work on macOS. Linux support requires Docker."; \
        exit 0; \
    fi
    cd aoc-2024-12-01/bindings/swift && \
    swiftc -o test_swift ../../tests/swift/test_swift_bindings.swift aoc_ffi_day01.swift \
        -import-objc-header aoc_ffi_day01FFI.h -L . -laoc_ffi_day01 && \
    LD_LIBRARY_PATH=. ./test_swift && \
    rm -f test_swift
    @echo ""
    @echo "✓ Swift tests completed"

# Generate all language bindings
uniffi-gen-all: uniffi-gen-python uniffi-gen-kotlin uniffi-gen-swift
    @echo "✓ All bindings generated!"

# Backward compatibility aliases
uniffi-gen: uniffi-gen-python
uniffi-test: uniffi-test-python uniffi-test-kotlin
uniffi-test-all: uniffi-test-python uniffi-test-kotlin uniffi-test-swift