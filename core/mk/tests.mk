# cargo

cargo-test:
	cargo test -- --test-threads=1

cargo-test-backtraced:
	RUST_BACKTRACE=1 cargo test -- --test-threads=1

# analyzer

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

# debugger

test-debugger:
	cargo run -q --bin husky-debugger -- --package-dir $(test_examples_dir)/debugger --sample-id 23 --mode test

test-debugger-v:
	cargo run -q --bin husky-debugger -- --package-dir $(test_examples_dir)/debugger -v --sample-id 23 --mode test

test-debugger-with-backtrace:
	RUST_BACKTRACE=1 cargo run -q --bin husky-debugger -- --package-dir $(test_examples_dir)/debugger --sample-id 23 --mode test

test-debugger-with-backtrace-filtered:
	RUST_BACKTRACE=1 cargo run -q --bin husky-debugger -- --package-dir $(test_examples_dir)/debugger --sample-id 23 --mode test 2>&1 | python scripts/filter_rust_backtrace.py

# compiler
