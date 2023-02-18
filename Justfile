run-app:
    cargo run --bin esfxr-app

run-tui:
    cargo run --bin esfxr-tui

run-web:
    trunk serve ./crates/esfxr-web/index.html

fmt:
    cargo fmt --all

lint:
    cargo clippy --all --tests
