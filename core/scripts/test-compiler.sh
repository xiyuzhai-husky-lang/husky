#!/bin/bash
set -e
if [ -z $HUSKY_DIR ]; then
    echo env not set
    exit
fi
TEST_DIR=${HUSKY_DIR}/tests
cd $HUSKY_DIR/core
cargo check
cargo run -q \
    --bin husky-compiler \
    -- \
    $TEST_DIR/compiler
# cd $RUST_GEN_CACHE_DIR && cargo fmt
# DIFF_RESULT=$(diff -r $RUST_GEN_CACHE_DIR/crates/tests $RUST_GEN_DIR/crates/tests >difference && echo same || echo different)
# if [ $DIFF_RESULT = "different" ]; then
#     rsync -a --delete $RUST_GEN_CACHE_DIR/crates/tests/ $RUST_GEN_DIR/crates/tests
# fi
# /bin/rm -rf $RUST_GEN_CACHE_DIR
# cd $RUST_GEN_DIR
# cargo check
# cargo build
# cargo test -- --nocapture
