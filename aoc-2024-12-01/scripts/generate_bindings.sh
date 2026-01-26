#!/usr/bin/env bash
# Generate Python bindings using uniffi

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "Building Rust library..."
cargo build --release --lib

echo "Generating Python bindings..."

# Create output directory
mkdir -p bindings/python

# Use cargo with uniffi CLI feature to generate bindings
cargo run --features uniffi/cli --bin uniffi-bindgen -- \
    generate \
    src/aoc_ffi_day01.udl \
    --library target/release/libaoc_ffi_day01.so \
    --language python \
    --out-dir bindings/python

echo "✓ Python bindings generated in bindings/python/"
echo ""
echo "To use the bindings:"
echo "  export PYTHONPATH=$PROJECT_ROOT/bindings/python:\$PYTHONPATH"
echo "  python3 tests/python/test_python_bindings.py"
