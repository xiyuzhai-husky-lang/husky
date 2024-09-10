#!/usr/bin/env python3

import os


def clear_empty_folders(path):
    # Walk the directory tree from bottom to top
    for root, dirs, files in os.walk(path, topdown=False):
        for dir in dirs:
            dir_path = os.path.join(root, dir)
            try:
                # Remove the directory if it is empty
                os.rmdir(dir_path)
                print(f"Removed empty folder: {dir_path}")
            except OSError:
                # Directory is not empty
                pass


if __name__ == "__main__":
    directory = "."
    clear_empty_folders(directory)
