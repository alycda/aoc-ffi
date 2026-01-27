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

### Swift Installation

- **macOS**: Swift is installed via Nix (included in `shell.nix` on Darwin only)
- **Linux/devcontainer**: Swift 6.2.3 is installed from Swift.org tarball during container setup
  - Nix Swift package fails to build on Linux, so we use official Swift.org binaries instead
  - Supports both x86_64 and aarch64 architectures
  - Installation happens automatically in `.devcontainer/setup.sh`

### Quick Fix When Switching Platforms

```bash
# Clean all generated bindings
just clean-bindings

# Rebuild and test for current platform
just uniffi-test
```

### Swift Installation and Testing

- **macOS**: Swift is installed via Nix (included in `shell.nix`)
  - `just uniffi-test-swift` runs tests natively

- **Linux/devcontainer**: Swift tests are **currently skipped** due to glibc conflicts
  - Nix provides glibc 2.42, but system Swift requires glibc 2.39
  - Error: `symbol lookup error: undefined symbol: __tunable_is_initialized`
  - `just uniffi-test-swift` detects Linux + Nix and skips tests gracefully
  - Swift bindings can be generated but not tested in devcontainer

**Note**: This is a known limitation. Future work may add Docker-based testing.

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