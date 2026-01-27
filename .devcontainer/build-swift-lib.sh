#!/usr/bin/env bash
set -euo pipefail

# Build Rust library for Swift without Nix environment
# This avoids glibc conflicts between Nix and system Swift

echo "Building Rust library for Swift (outside Nix environment)..."

# Clear Nix environment variables
unset IN_NIX_SHELL
unset NIX_CC NIX_BINTOOLS NIX_LDFLAGS NIX_CFLAGS_COMPILE
unset NIX_ENFORCE_NO_NATIVE NIX_HARDENING_ENABLE
unset PATH_LOCALE NIX_BUILD_CORES

# Use system PATH (ensure cargo is available)
export PATH="/usr/local/bin:/usr/bin:/bin:$HOME/.cargo/bin"

cd "$(dirname "$0")/../aoc-2024-12-01"
cargo build --release --lib --no-default-features

echo "✓ Library built successfully with system glibc"
