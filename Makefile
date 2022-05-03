projects_dir=/home/xiyuzhai/Documents/husky/projects
examples_dir=/home/xiyuzhai/Documents/husky/examples
test_examples_dir=/home/xiyuzhai/Documents/husky/test-examples
temp_dir=/home/xiyuzhai/Documents/husky/examples/good/input


test-examples:
	cargo run --bin husky-lang-compiler $(examples_dir)/good
	cargo run --bin husky-lang-debugger $(examples_dir) --input-id 1 --mode test

test-examples-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(examples_dir) --input-id 1 --mode test 2>&1 | python scripts/filter_rust_backtrace.py

test-temp:
	cargo run --bin husky-lang-compiler $(temp_dir)
	cargo run --bin husky-lang-debugger $(temp_dir) --input-id 1 --mode test


test-analyzer:
	cargo run --bin husky-analyzer-tester test-diagnostics $(test_examples_dir)/analyzer/diagnostics
	cargo run --bin husky-analyzer-tester test-folding-ranges $(test_examples_dir)/analyzer/folding-ranges
	cargo run --bin husky-analyzer-tester test-semantic-tokens $(test_examples_dir)/analyzer/semantic-tokens

vscode: test-analyzer
	scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer ~/.vscode/extensions/
	cargo install --path crates/apps/analyzer --bin husky-analyzer-server

mnist:
	cargo run --bin husky-lang-debugger $(projects_dir)/cv/mnist-classifier --input-id 11 --mode run

mnist-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(projects_dir)/cv/mnist-classifier --input-id 11 --mode run 2>&1 | python scripts/filter_rust_backtrace.py
