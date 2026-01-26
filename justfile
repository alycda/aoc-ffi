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

# Generate all language bindings
uniffi-gen-all: uniffi-gen-python uniffi-gen-kotlin
    @echo "✓ All bindings generated!"

# Backward compatibility aliases
uniffi-gen: uniffi-gen-python
uniffi-test: uniffi-test-python uniffi-test-kotlin