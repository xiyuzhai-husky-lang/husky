#!/usr/bin/python

import os
import shutil

mnist_dir = "examples/cv/mnist-classifier"
assert os.path.exists(mnist_dir)

diagnostics_misc_dir = "tests/analyzer/diagnostics/misc"
assert os.path.exists(diagnostics_misc_dir)

mnist_dir_pattern = "mnist-classifier"

largest_idx = 0

for entry in os.listdir(diagnostics_misc_dir):
    if entry.startswith(mnist_dir_pattern):
        largest_idx = max(
            largest_idx, int(entry[len(mnist_dir_pattern) : len(mnist_dir_pattern) + 3])
        )

new_idx = largest_idx + 1

save_dir = "./tests/analyzer/diagnostics/misc/mnist-classifier{:03d}".format(new_idx)

while True:
    answer = input(
        "Do you want to save current version of mnist-classifier\n  into `{}`? yes(y) or no(n): ".format(
            save_dir
        )
    )
    if answer == "yes" or answer == "y":
        shutil.copytree(
            mnist_dir,
            save_dir,
            ignore=lambda _, names: [name for name in names if name.endswith("hsk")],
        )
        break
    elif answer == "no" or answer == "n":
        print("nothing happens")
        break
    else:
        print("Please enter yes(y) or no(n).")
