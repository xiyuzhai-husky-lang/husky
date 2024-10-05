#!/usr/bin/env python
# it doesn't quite work now!!!
# its behavior is buggy
# and it doesn't check git cleaness properly.

import os
import sys
import shutil
import re
import subprocess

# Usage: python rename_rust_package.py <old_package_name> <new_package_name>


def rename_rust_package(old_package_name, new_package_name):
    # Find the package directory
    package_dir = None
    for root, dirs, _ in os.walk("."):
        if old_package_name in dirs:
            package_dir = os.path.join(root, old_package_name)
            break

    if package_dir is None:
        print(f"Error: Package '{old_package_name}' not found in the workspace.")
        sys.exit(1)

    # Rename the package directory
    new_package_dir = os.path.join(os.path.dirname(package_dir), new_package_name)
    shutil.move(package_dir, new_package_dir)

    # Update references in Cargo.toml files
    for root, _, files in os.walk("."):
        for file_name in files:
            if file_name == "Cargo.toml":
                file_path = os.path.join(root, file_name)
                with open(file_path, "r") as file:
                    content = file.read()
                if old_package_name in content:
                    content = re.sub(
                        old_package_name,
                        new_package_name,
                        content,
                    )
                    with open(file_path, "w") as file:
                        file.write(content)

    # Update references in Rust source files (.rs)
    for root, _, files in os.walk(new_package_dir):
        for file_name in files:
            if file_name.endswith(".rs"):
                file_path = os.path.join(root, file_name)
                with open(file_path, "r") as file:
                    content = file.read()
                if old_package_name in content:
                    content = re.sub(
                        rf'\b{re.escape(old_package_name).replace("-", "_")}\b',
                        new_package_name.replace("-", "_"),
                        content,
                    )
                    with open(file_path, "w") as file:
                        file.write(content)

    print(
        f"Package renamed from '{old_package_name}' to '{new_package_name}' successfully."
    )


def is_git_repo_clean():
    try:
        # Check for uncommitted changes
        subprocess.run(["git", "diff", "--quiet", "--exit-code"], check=True)

        # Check for staged changes
        subprocess.run(
            ["git", "diff", "--staged", "--quiet", "--exit-code"], check=True
        )

        # Check for untracked files
        result = subprocess.run(
            ["git", "ls-files", "--others", "--exclude-standard"],
            capture_output=True,
            text=True,
            check=True,
        )
        if result.stdout.strip():
            return False

        return True
    except subprocess.CalledProcessError:
        return False


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(
            "Usage: python rename_rust_package.py <old_package_name> <new_package_name>"
        )
        sys.exit(1)

    if not is_git_repo_clean():
        print(
            "Error: Git repository is not clean. Please commit or stash your changes."
        )
        sys.exit(1)

    old_package_name = sys.argv[1]
    new_package_name = sys.argv[2]
    rename_rust_package(old_package_name, new_package_name)
