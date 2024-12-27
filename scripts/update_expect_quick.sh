#!/bin/bash
set -e

. .local/env.sh

# Loop through each target and run tests
for TARGET in $MAKE_QUICK_TARGETS; do
  cargo build -p $TARGET --tests
  if ! (UPDATE_EXPECT=1 cargo test -p "$TARGET" -j 1 -- --nocapture && cargo test -p "$TARGET" -j 1); then
    scripts/play_update_expect_failure_music.sh
    exit 1  # Exit the script with an error code
  fi
done