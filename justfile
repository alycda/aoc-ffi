_default: 
    @just --list

rebuild:
    home-manager switch -b backup -f .devcontainer/home.nix

export USER := shell("whoami")

cheats:
    cheat -l

present:
    presenterm slides.md

present-with-speaker-notes:
    tmux kill-session -t present 2>/dev/null || true
    tmux new-session -d -s present 'presenterm --listen-speaker-notes slides.md' \; \
        split-window -h 'presenterm --publish-speaker-notes slides.md' \; \
        attach -t present

# C++ build and run
[working-directory: 'aoc-2024-12-01']
cpp-build:
    make

[working-directory: 'aoc-2024-12-01']
cpp-run: cpp-build
    ./main

[working-directory: 'aoc-2024-12-01']
cpp-clean:
    make clean

# Rust build and run
[working-directory: 'aoc-2024-12-01']
rust-run:
    cargo run --release

# Compare all implementations
compare: cpp-run rust-run