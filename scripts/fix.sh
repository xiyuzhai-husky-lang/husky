#!/bin/bash

# Duration over which to change the brightness, in seconds
DURATION=0.5

# Define a cleanup function
cleanup() {
  # Reset screen brightness
  scripts/adjust_screen_brightness.sh 1.0 HDMI-1-0 $DURATION
}

# Trap the EXIT signal to call the cleanup function
# This ensures cleanup runs on any exit, normal or error
trap cleanup EXIT

# Set the screen brightness to very low
scripts/adjust_screen_brightness.sh 0.0 HDMI-1-0 $DURATION

# Format the Rust project
cargo fix

# Run tests, handling failure and success with respective scripts
cargo test --features "allow-print" -- --nocapture \
  && scripts/play_update_expect_success_music.sh \
  || scripts/play_update_expect_failure_music.sh
