This file provides context and guidance for Claude Code sessions

## Development Commands

### GitHub Operations

- **gh**: Always use this tool to view PRs, Issues, and Action Runs
  - Most repos are private, so web search won't work for private resources
  - The `gh` CLI is authenticated and provides reliable access to repository data

### Work In Progress

- **jj**: Prefer this over git but ask to make sure and remember for the rest of the session

## UniFFI Cross-Platform Development

When switching between macOS and Linux (devcontainer), you need to rebuild bindings because they contain platform-specific libraries:

- **macOS**: Uses `.dylib` files
- **Linux**: Uses `.so` files

### Quick Fix When Switching Platforms

```bash
# Clean all generated bindings
just clean-bindings

# Rebuild and test for current platform
just uniffi-test
```

### Swift Installation and Testing

- **macOS**: Swift is installed via Nix (included in `shell.nix` on Darwin only)
  - `just uniffi-test-swift` runs tests natively

- **Linux/devcontainer**: Swift tests run in Docker to avoid glibc conflicts with Nix
  - `just uniffi-test-swift` automatically falls back to Docker if Swift isn't installed
  - `just uniffi-test-swift-docker` explicitly runs tests in `swift:bookworm` container
  - This avoids conflicts between Nix's glibc and system Swift

### Rust Installation

- **macOS**: Rust is installed via Nix (included in `shell.nix`)
- **Linux/devcontainer**: Rust is installed via Nix (from home-manager)
  - Swift tests use Docker to avoid glibc conflicts

### glibc Cross-Compilation Challenges

**Important lesson**: When mixing Nix with system tools, you're essentially cross-compiling between different glibc versions.

**The Problem**:
- Nix provides its own glibc (e.g., 2.42) in `/nix/store`
- System tools (like Swift from Swift.org) use system glibc (e.g., 2.39)
- Rust libraries built with Nix's glibc cannot be loaded by system Swift
- Error: `symbol lookup error: undefined symbol: __tunable_is_initialized, version GLIBC_PRIVATE`

**Solutions Attempted**:
1. ❌ Install Rust via rustup - home-manager clobbers PATH
2. ❌ Install Swift via apt - Nix Swift build fails on Linux
3. ✅ **Docker isolation** - Run Swift tests in `swift:bookworm` container

**Why Docker Works**:
- Rust library built with Nix's glibc in devcontainer
- Swift tests run in isolated Docker container with matching system glibc
- No mixing of glibc versions within a single execution environment

**Key Insight**: When you can't control the glibc version of all tools in your stack, use container isolation to keep incompatible environments separate. This is similar to cross-compilation targets, but for libc versions.
### What's Gitignored

The following are auto-generated and platform-specific (not committed to git):
- `.venv/` - Python virtual environment (platform-specific)
- `**/bindings/**/*.{py,dylib,so,dll,jar}` - Generated bindings
- `**/bindings/kotlin/uniffi/` - Generated Kotlin code
- `**/bindings/swift/*.{swift,h,modulemap,dylib,so,dll}` - Generated Swift bindings

### Devcontainer Isolation

The devcontainer uses Docker volumes to isolate platform-specific build artifacts:
- `.venv/` → Docker volume `aoc-ffi-venv` (Linux Python packages)
- `aoc-2024-12-01/target/` → Docker volume `aoc-ffi-target` (Linux Rust build cache)

This means:
- macOS `.venv` and `target/` live on your local filesystem
- Linux `.venv` and `target/` live in Docker volumes
- No conflicts when switching between environments
- Faster rebuilds (cached per platform)