{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    just cheat asciinema_3 presenterm tmux bacon
    cargo # rustc 
    gcc gnumake clang llvmPackages.libclang.lib
    # FFI dependencies for glib-sys
    pkg-config glib
    # Python for UniFFI bindings
    python3
    python3Packages.pip
    python3Packages.virtualenv
    # JDK and Kotlin for UniFFI Kotlin bindings
    jdk17
    kotlin
    # Swift for UniFFI Swift bindings
    # Note: Swift is installed via system package manager on Linux (see .devcontainer/setup.sh)
    # On macOS, install Swift via: brew install swift or use Xcode
  ] ++ lib.optionals stdenv.isDarwin [
    swift
    rustc # installed in devcontainer to prevent conflicting glibc for swift
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  shellHook = ''
    # Prevent SDK conflicts by unsetting DEVELOPER_DIR_FOR_TARGET
    unset DEVELOPER_DIR_FOR_TARGET
    unset NIX_APPLE_SDK_VERSION_FOR_TARGET

    # Set up Python virtual environment for UniFFI
    if [ ! -d ".venv" ]; then
      echo "Creating Python virtual environment..."
      python3 -m venv .venv --copies
    fi

    # Activate virtual environment
    if [ -f ".venv/bin/activate" ]; then
      source .venv/bin/activate
    else
      echo "⚠ Warning: .venv exists but activate script not found"
      echo "  Recreating virtual environment..."
      rm -rf .venv
      python3 -m venv .venv --copies
      source .venv/bin/activate
    fi

    # Verify we're in the venv (should not be in /nix/store)
    if [[ "$(which python)" == *".venv"* ]]; then
      # Install uniffi-bindgen if not present or wrong version
      if ! python -c "import uniffi_bindgen; assert uniffi_bindgen.__version__ == '0.28.3'" 2>/dev/null; then
        echo "Installing uniffi-bindgen 0.28.3..."
        pip install --upgrade pip setuptools 2>/dev/null
        pip install uniffi-bindgen==0.28.3
      fi
    else
      echo "⚠ Warning: Virtual environment not properly activated"
      echo "  Python path: $(which python)"
      echo "  Try exiting and re-entering the nix-shell"
    fi

    # Download JNA for Kotlin UniFFI bindings if not present
    JNA_VERSION="5.13.0"
    JNA_DIR="$HOME/.m2/repository/net/java/dev/jna/jna/$JNA_VERSION"
    JNA_JAR="$JNA_DIR/jna-$JNA_VERSION.jar"

    if [ ! -f "$JNA_JAR" ]; then
      echo "Downloading JNA library $JNA_VERSION..."
      mkdir -p "$JNA_DIR"
      curl -L -o "$JNA_JAR" "https://repo1.maven.org/maven2/net/java/dev/jna/jna/$JNA_VERSION/jna-$JNA_VERSION.jar"
      echo "✓ JNA library downloaded"
    fi

    # Set CLASSPATH for Kotlin JNA dependency
    export CLASSPATH="$JNA_JAR:$CLASSPATH"

    echo "✓ Rust FFI development environment ready"
    echo "  - Rust toolchain: $(rustc --version 2>/dev/null || echo 'installing...')"
    echo "  - Python $(python --version 2>&1 | cut -d' ' -f2) with uniffi-bindgen"
    echo "  - Kotlin $(kotlin -version 2>&1 | head -n1)"
    if command -v swift &> /dev/null; then
      echo "  - Swift $(swift --version 2>&1 | head -n1 | cut -d' ' -f4)"
    else
      echo "  - Swift: not installed (install via system package manager)"
    fi

    echo "UniFFI environment ready"
    echo ""
    echo "Python bindings:"
    echo "  just uniffi-gen-python  # Generate Python bindings"
    echo "  just uniffi-test-python # Test Python bindings"
    echo ""
    echo "Kotlin bindings:"
    echo "  just uniffi-gen-kotlin  # Generate Kotlin bindings"
    echo "  just uniffi-test-kotlin # Test Kotlin bindings"
    echo ""
    echo "Swift bindings:"
    echo "  just uniffi-gen-swift   # Generate Swift bindings"
    if [ "$(uname)" = "Linux" ]; then
      echo "  just uniffi-test-swift  # Test Swift bindings (skipped on Linux/Nix)"
    else
      echo "  just uniffi-test-swift  # Test Swift bindings"
    fi
    echo ""
    echo "All bindings:"
    echo "  just uniffi-gen-all     # Generate all bindings"
    echo "  just uniffi-test        # Test Python + Kotlin (Swift on macOS only)"
  '';
}