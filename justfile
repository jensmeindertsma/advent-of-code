help:
    just --list

test puzzle="":
    #!/usr/bin/env bash
    if [ -n "{{puzzle}}" ]; then
        cargo nextest run --release --package puzzle-{{puzzle}}
    else
        cargo nextest run --release
    fi
