projects_dir=/home/xiyuzhai/Documents/husky/projects
examples_dir=/home/xiyuzhai/Documents/husky/examples
test_examples_dir=/home/xiyuzhai/Documents/husky/test-examples
temp_dir=/home/xiyuzhai/Documents/husky/examples/good/input


test-examples:
	cargo run --bin husky-lang-compiler $(examples_dir)/good
	cargo run --bin husky-lang-debugger $(examples_dir) --input-id 1 --mode test

test-examples-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(examples_dir) --input-id 1 --mode test 2>&1 | python scripts/filter_rust_backtrace.py

cargo-test:
	cargo test

test-analyzer: cargo-test
	cargo run -q --bin husky-analyzer-tester test-diagnostics $(test_examples_dir)/analyzer/diagnostics
	cargo run -q --bin husky-analyzer-tester test-folding-ranges $(test_examples_dir)/analyzer/folding-ranges
	cargo run -q --bin husky-analyzer-tester test-semantic-tokens $(test_examples_dir)/analyzer/semantic-tokens
	cargo run -q --bin husky-analyzer-tester test-qualified-tys $(test_examples_dir)/analyzer/qualified-tys

test-debugger: cargo-test
	cargo run -q --bin husky-lang-debugger $(test_examples_dir)/debugger --input-id 11 --mode test

test-temp:
	cargo run -q --bin husky-lang-debugger $(test_examples_dir)/debugger/proc/loop2 --input-id 11 --mode run

test-debugger-with-backtrace:
	RUST_BACKTRACE=1 cargo run -q --bin husky-lang-debugger $(test_examples_dir)/debugger --input-id 11 --mode test
	
#	| python scripts/filter_rust_backtrace.py

vscode: test-analyzer
	scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer ~/.vscode/extensions/
	cargo install --path crates/apps/analyzer --bin husky-analyzer-server

mnist:
	cargo run -q --bin husky-lang-debugger $(projects_dir)/cv/mnist-classifier --input-id 11 --mode run

print-mnist:
	cargo run -q --bin husky-analyzer-printer print-qualified-tys $(projects_dir)/cv/mnist-classifier

mnist-with-backtrace:
	RUST_BACKTRACE=1 cargo run -q --bin husky-lang-debugger $(projects_dir)/cv/mnist-classifier --input-id 11 --mode run 2>&1 | python scripts/filter_rust_backtrace.py
