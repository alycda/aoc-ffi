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

[working-directory: 'aoc-2024-12-01']
aoc-test:
    cargo test

[working-directory: 'aoc-2024-12-01']
aoc-bench:
    cargo bench

[working-directory: 'aoc-2024-12-01']
aoc-run:
    cargo run

# UniFFI bindings - build library
[working-directory: 'aoc-2024-12-01']
build-lib:
    cargo build --release --lib