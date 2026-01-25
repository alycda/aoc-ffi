{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [ just cheat asciinema_3 presenterm tmux bacon cargo rustc gcc gnumake clang llvmPackages.libclang.lib ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}