#!/bin/bash

# Format the Rust project
cargo fmt

# update jar tree information
UPDATE_EXPECT=1 cargo test -p husky-jar-utils

# Check the Rust project, including tests
cargo check --tests

# Run tests, handling failure and success with respective scripts
UPDATE_EXPECT=1 cargo test --features "allow-print" -- --nocapture \
  && scripts/play_update_expect_success_music.sh \
  || scripts/play_update_expect_failure_music.sh
