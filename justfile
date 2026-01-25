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

# AOC benchmarks
aoc-test:
    cd aoc-2024-12-01 && cargo test

aoc-bench:
    cd aoc-2024-12-01 && cargo bench

aoc-run:
    cd aoc-2024-12-01 && cargo run