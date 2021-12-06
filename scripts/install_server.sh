set -e
cargo build --package syntax && cargo test --package syntax
cargo install --path crates/husky_lang_server
