help:
    just --list

check:
    cargo clippy

debug puzzle:
    cargo run --package puzzle-{{puzzle}}

format:
    cargo fmt --all

solve puzzle:
    cargo run --release --package puzzle-{{puzzle}}

test puzzle="":
    #!/usr/bin/env bash
    if [ -n "{{puzzle}}" ]; then
        cargo nextest run --release --package puzzle-{{puzzle}}
    else
        cargo nextest run --release
    fi
