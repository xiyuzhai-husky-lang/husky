examples_dir=examples
tests_dir=tests

include mk/*.mk

update-python-requirements:
	pipreqs ./ --force
