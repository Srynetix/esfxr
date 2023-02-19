index_html_path := "./crates/frontends/esfxr-egui-web/index.html"

_default:
    just --list

# Run the desktop app
run-desktop:
    cargo run --bin esfxr-egui-desktop

# Run the TUI app
run-tui:
    cargo run --bin esfxr-tui

# Run the Web app
run-web:
    trunk serve {{ index_html_path }}

# Build the web app
build-web:
    trunk build {{ index_html_path }}

# Build all (minus the web app)
build:
    cargo build

# Format all
fmt:
    cargo fmt --all

# Lint all
lint:
    cargo clippy --all --tests
