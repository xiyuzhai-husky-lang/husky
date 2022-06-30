#!/usr/bin/python

import os
import shutil

husky_dir = os.getenv("HUSKY_DIR")
assert os.getcwd() == husky_dir

mnist_dir = os.path.join(husky_dir, "projects/cv/mnist-classifier")
assert os.path.exists(mnist_dir)

folding_ranges_misc_dir = os.path.join(
    husky_dir, "test-examples/analyzer/folding-ranges/misc"
)
assert os.path.exists(folding_ranges_misc_dir)

mnist_dir_pattern = "mnist-classifier"

largest_idx = 0

for entry in os.listdir(folding_ranges_misc_dir):
    if entry.startswith(mnist_dir_pattern):
        largest_idx = max(
            largest_idx, int(entry[len(mnist_dir_pattern) : len(mnist_dir_pattern) + 3])
        )

new_idx = largest_idx + 1

save_dir = "./test-examples/analyzer/folding-ranges/misc/mnist-classifier{:03d}".format(
    new_idx
)

while True:
    answer = input(
        "Do you want to save current version of mnist-classifier\n  into `{}`? yes(y) or no(n): ".format(
            save_dir
        )
    )
    if answer == "yes" or answer == "y":
        shutil.copytree(mnist_dir, save_dir)
        break
    elif answer == "no" or answer == "n":
        print("nothing happens")
        break
    else:
        print("Please enter yes(y) or no(n).")
