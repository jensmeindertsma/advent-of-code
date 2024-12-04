help:
    just --list

debug puzzle:
    cargo nextest run --package puzzle-{{puzzle}}

solve puzzle:
    cargo run --release --package puzzle-{{puzzle}}

test puzzle="":
    #!/usr/bin/env bash
    if [ -n "{{puzzle}}" ]; then
        cargo nextest run --release --package puzzle-{{puzzle}}
    else
        cargo nextest run --release
    fi

