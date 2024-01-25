_all:
    @just --list

# Format code
fmt:
    cargo fmt

# Lint code
lint:
    cargo clippy --all

# Install
install:
    cargo install --path crates/qx