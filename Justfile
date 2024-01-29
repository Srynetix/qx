_all:
    @just --list

# Format code
fmt:
    cargo fmt

# Lint code
lint:
    cargo clippy --all

# Build
build:
    cargo build

# Install
install:
    cargo install --path crates/qx

# Test
test:
    cargo test --all

# Changelog
changelog:
    git cliff -o CHANGELOG.md

# Bump versions
bump-version version:
    cargo xtask bump-version {{ version }}