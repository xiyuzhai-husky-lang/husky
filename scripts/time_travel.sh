#!/bin/bash

# Git Time Travel Script
# ======================
# This script allows you to "time travel" a Git repository by checking out the commit that is closest to a specified date.
# It is particularly useful when you want to explore the state of the codebase at a given point in time.
#
# Intended Purpose:
# ------------------
# - The script checks out the commit just before or on the specified date.
# - You can use this to inspect older versions of the codebase or run tests on past states of the project.
#
# Usage:
# ------
# - Run the script by providing a date as an argument in 'YYYY-MM-DD' format.
# - The script will look for the most recent commit before that date and checkout to it.
# - If no commit exists before the date, the script will exit with an error message.
#
# Example:
# --------
# ./git-time-travel.sh '2023-01-15'
#
# Dependencies:
# -------------
# - Git must be installed and initialized in the current working directory.
#
# The following script also provides a help menu accessible with the `-h` or `--help` option.

# Check for help option
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    echo "Git Time Travel Script"
    echo ""
    echo "Usage: $0 <date>"
    echo "Example: $0 '2023-01-15'"
    echo ""
    echo "This script checks out the closest commit before or on the given date."
    echo ""
    echo "Options:"
    echo "  -h, --help    Show this help message"
    exit 0
fi

# Check if the user provided a date
if [ -z "$1" ]; then
    echo "Error: No date provided."
    echo "Usage: $0 <date>"
    echo "Example: $0 '2023-01-15'"
    exit 1
fi

# Store the input date
TARGET_DATE="$1"

# Fetch the latest changes from the remote repository (optional, you can remove if not needed)
git fetch --all

# Get the closest commit to the specified date
COMMIT_HASH=$(git rev-list -n 1 --before="$TARGET_DATE" --all)

# Check if a commit was found
if [ -z "$COMMIT_HASH" ]; then
    echo "No commit found before $TARGET_DATE"
    exit 1
fi

# Checkout the commit
git checkout "$COMMIT_HASH"

echo "Repository has been time traveled to the commit on or before $TARGET_DATE"
