default: test-all
test year day:
    cargo nextest run --release --no-fail-fast --final-status-level all --package puzzle-{{year}}-{{day}}
test-all:
    cargo nextest run --release --no-fail-fast --final-status-level all --failure-output never
solve year day:
    cargo run --release --package puzzle-{{year}}-{{day}}
