set quiet

help:
  just --list

clean:
  cargo clean

check:
  cargo clippy --workspace

debug puzzle:
  cargo run --package puzzle-{{puzzle}}

format:
  cargo fmt --all

format-ci:
  cargo fmt --all --check

solve puzzle:
  cargo run --release --quiet --package puzzle-{{puzzle}}

test puzzle:
  cargo nextest run --release --package puzzle-{{puzzle}}

test-all:
  cargo nextest run --release
