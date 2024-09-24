#!/bin/bash

# Ensure scc is installed
if ! command -v scc &> /dev/null
then
    echo "scc could not be found, please install it."
    exit 1
fi

# Set the base directory (change 'crates' if needed)
BASE_DIR="crates"

# Iterate over each subfolder in the 'crates' directory
for dir in "$BASE_DIR"/*/ ; do
    # Run scc on the subdirectory, excluding paths with "expect-files"
    echo "Counting lines in $dir (ignoring 'expect-files' paths)"
    scc "$dir" --exclude-dir "expect-files"
done
