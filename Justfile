index_html_path := "./crates/frontends/esfxr-egui-web/index.html"

_default:
    just --list

# Run the desktop app
run-desktop:
    cargo run --release --bin esfxr-egui-desktop

# Run the TUI app
run-tui:
    cargo run --release --bin esfxr-tui

# Run the Web app
run-web:
    trunk serve --release {{ index_html_path }}

# Build the web app
build-web:
    trunk build --release {{ index_html_path }}

# Build all (minus the web app)
build:
    cargo build --release

# Format all
fmt:
    cargo fmt --all

# Check format
fmt-check:
    cargo fmt --check --all

# Lint all
lint:
    cargo clippy --all --tests

# Test
test:
    cargo test --all

# CI
ci:
    just fmt-check && just lint && just test && cargo build
