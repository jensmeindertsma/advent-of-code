help:
    just --list

check *FLAGS:
    cargo clippy {{FLAGS}}

debug puzzle:
    cargo run --package puzzle-{{puzzle}}

format *FLAGS:
    cargo fmt --all {{FLAGS}}

solve puzzle:
    cargo run --release --package puzzle-{{puzzle}}

test puzzle="":
    #!/usr/bin/env bash
    if [ -n "{{puzzle}}" ]; then
        cargo nextest run --release --package puzzle-{{puzzle}}
    else
        cargo nextest run --release
    fi
