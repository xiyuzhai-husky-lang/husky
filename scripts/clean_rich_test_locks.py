#!/usr/bin/env python3

import os
from typing import Set

# High-level Explanation:
# This script processes all `.rich-test-lock` files under the `crates` directory.
# For each lock file, it collects the absolute paths listed in the file and the directory
# containing the lock file. After processing, it deletes the lock file.
# The script then checks if any files in `<lock-file-parent-dir>/expect-files` and
# `<lock-file-parent-dir>/adversarials` do not belong to the set of covered paths and deletes them.
# Finally, it deletes any empty directories in those locations.

# Global sets to store covered paths and directories containing lock files
covered_paths: Set[str] = set()
dirs_with_lock_files: Set[str] = set()


def process_lock_file(lock_file_path: str) -> None:
    """Process a single .rich-test-lock file to extract paths and store the parent directory."""
    with open(lock_file_path, "r", encoding="utf8") as lock_file:
        for line in lock_file:
            covered_paths.add(line.strip())

    # Add the parent directory of the lock file to the set of directories
    parent_dir = os.path.dirname(lock_file_path)
    dirs_with_lock_files.add(parent_dir)

    # Delete the lock file after processing
    os.remove(lock_file_path)
    print(f"Deleted lock file: {lock_file_path}")


def clean_directory(dir_path: str) -> None:
    """Clean unnecessary files and directories
    under <dir_path>/expect-files and <dir_path>/adversarials."""
    for sub_dir in ["expect-files", "adversarials"]:
        target_dir = os.path.join(dir_path, sub_dir)
        if os.path.exists(target_dir):
            # Remove files not in covered paths
            for root, _dirs, files in os.walk(target_dir):
                for file in files:
                    file_path = os.path.join(root, file)
                    if os.path.abspath(file_path) not in covered_paths:
                        os.remove(file_path)
                        print(f"Deleted file: {file_path}")

            # Remove empty directories
            for root, dirs, _files in os.walk(target_dir, topdown=False):
                for directory in dirs:
                    dir_to_check = os.path.join(root, directory)
                    if not os.listdir(dir_to_check):
                        os.rmdir(dir_to_check)
                        print(f"Deleted empty directory: {dir_to_check}")


def main() -> None:
    """Main function to process all lock files and clean directories."""
    # Define the base directory
    base_directory = "crates"

    # Process each .rich-test-lock file
    for root, _dirs, files in os.walk(base_directory):
        for file in files:
            if file.endswith(".rich-test-lock"):
                lock_file_path = os.path.join(root, file)
                process_lock_file(lock_file_path)

    # Clean directories that had lock files
    for dir_path in dirs_with_lock_files:
        clean_directory(dir_path)


if __name__ == "__main__":
    main()
