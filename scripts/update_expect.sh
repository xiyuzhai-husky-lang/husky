#!/bin/bash

# Duration over which to change the brightness, in seconds
DURATION=0.5
MONITOR=HDMI-2

# Define a cleanup function
cleanup() {
  # Reset screen brightness
  scripts/adjust_screen_brightness.sh 1.0 $MONITOR $DURATION &
}

# Trap the EXIT signal to call the cleanup function
# This ensures cleanup runs on any exit, normal or error
trap cleanup EXIT

# Set the screen brightness to very low
scripts/adjust_screen_brightness.sh 0.0 $MONITOR $DURATION &

# Format the Rust project
cargo fmt

# update jar tree information
UPDATE_EXPECT=1 cargo test -p husky-jar-utils

# Check the Rust project, including tests
cargo check --tests

# Run tests, handling failure and success with respective scripts
UPDATE_EXPECT=1 cargo test --features "allow-print" -j 1 -- --nocapture \
  && cargo test \
  && scripts/play_update_expect_success_music.sh \
  || scripts/play_update_expect_failure_music.sh
