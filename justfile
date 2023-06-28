# Run native
native:
    cargo run --release

# Run wasm
wasm:
    cargo install wasm-server-runner
    cargo run --release --target wasm32-unknown-unknown

# Create app bundle with icon (tested only on MacOS)
bundle:
    cargo build --release
    cargo install cargo-bundle
    cargo bundle

# Lints to be used before commit
lint:
    cargo fmt
    cargo clippy -- -A clippy::type_complexity -A clippy::too_many_arguments
