set -e
#cargo build --package syntax && cargo test --package syntax
cargo install --path crates/husky-lang/analyzer
cargo install --path crates/husky-lang/debugger
