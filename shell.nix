{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    just cheat asciinema_3 presenterm tmux bacon
    cargo rustc gcc gnumake clang llvmPackages.libclang.lib
    # FFI dependencies for glib-sys
    pkg-config glib
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}