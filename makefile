examples_dir=examples
tests_dir=tests

include tests/makefile
include examples/makefile

update-python-requirements:
	pipreqs ./ --force

vscode: cargo-test test-analyzer
	scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer ~/.vscode/extensions/
	cargo install --path crates/apps/husky-analyzer --bin husky-analyzer-server

install-compiler:
	cargo install --path crates/apps/husky-compiler --bin husky-compiler

count-todo:
	scripts/pattern_statistics.py "todo!()" crates 1 10
	scripts/pattern_statistics.py "todo!()" crates 2 10

ubuntu-setup:
	scripts/ubuntu_setup.sh