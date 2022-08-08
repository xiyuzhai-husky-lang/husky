examples_dir=examples
tests_dir=tests

include mk/*.mk
include tests/makefile

update-python-requirements:
	pipreqs ./ --force
