projects_dir=/home/xiyuzhai/Documents/husky/projects
examples_dir=/home/xiyuzhai/Documents/husky/examples
test_examples_dir=/home/xiyuzhai/Documents/husky/test-examples
temp_dir=/home/xiyuzhai/Documents/husky/examples/good/input


test-examples:
	cargo run --bin husky-compiler $(examples_dir)/good
	cargo run --bin husky-tracer $(examples_dir) --sample-idx 1 --mode test

test-examples-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-tracer $(examples_dir) --sample-idx 1 --mode test 2>&1 | python scripts/filter_rust_backtrace.py

cargo-test:
	cargo test

test-analyzer:
	cargo run -q --bin husky-analyzer-tester test-diagnostics $(test_examples_dir)/analyzer/diagnostics \
	&& cargo run -q --bin husky-analyzer-tester test-folding-ranges $(test_examples_dir)/analyzer/folding-ranges \
	&& cargo run -q --bin husky-analyzer-tester test-semantic-tokens $(test_examples_dir)/analyzer/semantic-tokens\
	&& cargo run -q --bin husky-analyzer-tester test-qualified-tys $(test_examples_dir)/analyzer/qualified-tys

test-analyzer-with-backtrace:
	set -e
	RUST_BACKTRACE=1 cargo run -q --bin husky-analyzer-tester test-diagnostics $(test_examples_dir)/analyzer/diagnostics
	RUST_BACKTRACE=1 cargo run -q --bin husky-analyzer-tester test-folding-ranges $(test_examples_dir)/analyzer/folding-ranges
	RUST_BACKTRACE=1 cargo run -q --bin husky-analyzer-tester test-semantic-tokens $(test_examples_dir)/analyzer/semantic-tokens
	RUST_BACKTRACE=1 cargo run -q --bin husky-analyzer-tester test-qualified-tys $(test_examples_dir)/analyzer/qualified-tys

test-analyzer-with-backtrace-filtered:
	set -e
	RUST_BACKTRACE=1 cargo run -q --bin husky-analyzer-tester test-diagnostics $(test_examples_dir)/analyzer/diagnostics 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run -q --bin husky-analyzer-tester test-folding-ranges $(test_examples_dir)/analyzer/folding-ranges 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run -q --bin husky-analyzer-tester test-semantic-tokens $(test_examples_dir)/analyzer/semantic-tokens 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run -q --bin husky-analyzer-tester test-qualified-tys $(test_examples_dir)/analyzer/qualified-tys 2>&1 | python scripts/filter_rust_backtrace.py

test-debugger:
	cargo run -q --bin husky-tracer $(test_examples_dir)/debugger --sample-idx 23 --mode test

test-debugger-v:
	cargo run -q --bin husky-tracer $(test_examples_dir)/debugger -v --sample-idx 23 --mode test

test-temp:
	cargo run -q --bin husky-tracer $(test_examples_dir)/debugger/proc/loop2 --sample-idx 23 --mode run

test-debugger-with-backtrace:
	RUST_BACKTRACE=1 cargo run -q --bin husky-tracer $(test_examples_dir)/debugger --sample-idx 23 --mode test
	
#	| python scripts/filter_rust_backtrace.py

vscode: cargo-test test-analyzer
	scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer ~/.vscode/extensions/
	cargo install --path crates/apps/analyzer --bin husky-analyzer-server

mnist:
	cargo run -q --bin husky-tracer $(projects_dir)/cv/mnist-classifier --sample-idx 23 --mode run

print-mnist:
	cargo run -q --bin husky-analyzer-printer print-qualified-tys $(projects_dir)/cv/mnist-classifier

mnist-with-backtrace:
	RUST_BACKTRACE=1 cargo run -q --bin husky-tracer $(projects_dir)/cv/mnist-classifier --sample-idx 23 --mode run

mnist-with-backtrace-filtered:
	RUST_BACKTRACE=1 cargo run -q --bin husky-tracer $(projects_dir)/cv/mnist-classifier --sample-idx 23 --mode run 2>&1 | python scripts/filter_rust_backtrace.py

count-todo:
	scripts/pattern_statistics.py "todo!()" crates 1 10
	scripts/pattern_statistics.py "todo!()" crates 2 10

update-requirements:
	pipreqs ./
