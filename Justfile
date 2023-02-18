app:
    cargo run --bin esfxr-app

tui:
    cargo run --bin esfxr-tui

fmt:
    cargo fmt --all

lint:
    cargo clippy --all --tests
