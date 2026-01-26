{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    just cheat asciinema_3 presenterm tmux bacon
    cargo rustc gcc gnumake clang llvmPackages.libclang.lib
    # FFI dependencies for glib-sys
    pkg-config glib
    # Python for UniFFI bindings
    python3
    python3Packages.pip
    python3Packages.virtualenv
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  shellHook = ''
    # Set up Python virtual environment for UniFFI
    if [ ! -d ".venv" ]; then
      echo "Creating Python virtual environment..."
      python3 -m venv .venv
    fi

    source .venv/bin/activate

    # Install uniffi-bindgen if not present or wrong version
    if ! python -c "import uniffi_bindgen; assert uniffi_bindgen.__version__ == '0.28.3'" 2>/dev/null; then
      echo "Installing uniffi-bindgen 0.28.3..."
      pip install --quiet uniffi-bindgen==0.28.3
    fi
  '';
}