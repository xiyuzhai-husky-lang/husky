#!/bin/bash
set -e
TEST_DIR=${HUSKY_DIR}/tests
RUST_GEN_DIR=$HUSKY_DIR/__rust_gen__
RUST_GEN_CACHE_DIR=$HUSKY_DIR/__rust_gen_cache__
mkdir -p $RUST_GEN_CACHE_DIR/crates/tests
echo [workspace] >$RUST_GEN_CACHE_DIR/Cargo.toml
echo members = [\"crates/tests/*\"] >>$RUST_GEN_CACHE_DIR/Cargo.toml
cd $HUSKY_DIR/core
cargo check
cargo run -q \
    --bin husky-compiler \
    -- \
    --src $TEST_DIR/compiler \
    --dst $RUST_GEN_CACHE_DIR/crates/tests \
    --rel-husky-dir "../../../.."
cd $RUST_GEN_CACHE_DIR && cargo fmt
DIFF=$(diff -r $RUST_GEN_CACHE_DIR/crates/tests $RUST_GEN_DIR/crates/tests)
if [ -n "$DIFF" ]; then
    rsync -r $RUST_GEN_CACHE_DIR/crates/tests/ $RUST_GEN_DIR/crates/tests
fi
/bin/rm -rf $RUST_GEN_CACHE_DIR
cd $RUST_GEN_DIR
cargo check
