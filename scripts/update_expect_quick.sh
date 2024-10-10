#!/bin/bash
set -e

. .local/env.sh

# Duration over which to change the brightness, in seconds
DURATION=0.5
MONITOR=HDMI-2

# # Define a cleanup function
# cleanup() {
#   # Reset screen brightness
#   # scripts/adjust_screen_brightness.sh 1.0 $MONITOR $DURATION &
# }

# Trap the EXIT signal to call the cleanup function
# This ensures cleanup runs on any exit, normal or error
# trap cleanup EXIT

# Set the screen brightness to very low
# scripts/adjust_screen_brightness.sh 0.0 $MONITOR $DURATION &

# Format the Rust project
cargo fmt

# update jar tree information
UPDATE_EXPECT=1 cargo test -p husky-jar-utils

# Check the Rust project, including tests
cargo check --tests

# Initialize a variable to track overall success
OVERALL_SUCCESS=true

# Loop through each target and run tests
for TARGET in $MAKE_QUICK_TARGETS; do
  cargo build -p $TARGET --tests
  if ! UPDATE_EXPECT=1 cargo test -p "$TARGET" -j 1 -- --nocapture; then
    OVERALL_SUCCESS=false
  fi
done

# Play success or failure music based on the overall result
if $OVERALL_SUCCESS; then
  scripts/play_update_expect_success_music.sh
else
  scripts/play_update_expect_failure_music.sh
fi
