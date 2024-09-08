#!/usr/bin/env python3

import os

# Define the directory to search in
directory = "crates"

# Walk through the directory
for root, dirs, files in os.walk(directory):
    for file in files:
        if file.endswith(".rich-test-lock"):
            file_path = os.path.join(root, file)
            try:
                os.remove(file_path)
            except Exception as e:
                print(f"Error deleting {file_path}: {e}")
