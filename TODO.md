# TODO

* use `utoipa` to generate openapi documents
* use rust documentation
```rust
#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs, clippy::unwrap_used)]
```
* cargo-tarpaulin

## windows installation experience

has to use `choco` to install make first

```sh
cargo install trunk
rustup target add wasm32-unknown-unknown
trunk serve
```
