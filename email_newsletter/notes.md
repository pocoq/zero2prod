cargo clean
cargo check
cargo run

cargo watch -x check
cargo test

cargo clippy
cargo clippy -- -D warnings

cargo fmt
cargo fmt -- --check
