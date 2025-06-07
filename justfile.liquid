# List all available commands
default:
    just --list

# Build the project
build:
    RUST_BACKTRACE=1 cargo build --workspace --all-features --tests --bins --benches

# Clean the build artifacts
clean:
    cargo clean --verbose

# Linting
clippy:
   cargo clippy --workspace --all-features --tests --bins --benches -- -D warnings

# Check formatting
check-fmt:
    cargo +nightly fmt --all -- --check

# Fix formatting
fmt:
    cargo +nightly fmt --all

# Test the project
test:
    RUST_BACKTRACE=1 cargo test --workspace --all-features --verbose

# Run all the checks
check:
    just check-fmt
    just clippy
    just test

# Run all commend in the local environment
all:
    just clean
    just check
    just build
