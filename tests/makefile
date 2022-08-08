# cargo

cargo-test:
	cargo test -- --test-threads=1

cargo-test-backtraced:
	RUST_BACKTRACE=1 cargo test -- --test-threads=1

# analyzer

test-analyzer:
	cargo run --bin husky-analyzer-tester test-diagnostics $(tests_dir)/analyzer/diagnostics \
	&& cargo run --bin husky-analyzer-tester test-folding-ranges $(tests_dir)/analyzer/folding-ranges \
	&& cargo run --bin husky-analyzer-tester test-semantic-tokens $(tests_dir)/analyzer/semantic-tokens\
	&& cargo run --bin husky-analyzer-tester test-qualified-tys $(tests_dir)/analyzer/qualified-tys

test-analyzer-with-backtrace:
	set -e
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-diagnostics $(tests_dir)/analyzer/diagnostics
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-folding-ranges $(tests_dir)/analyzer/folding-ranges
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-semantic-tokens $(tests_dir)/analyzer/semantic-tokens
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-qualified-tys $(tests_dir)/analyzer/qualified-tys

test-analyzer-with-backtrace-filtered:
	set -e
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-diagnostics $(tests_dir)/analyzer/diagnostics 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-folding-ranges $(tests_dir)/analyzer/folding-ranges 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-semantic-tokens $(tests_dir)/analyzer/semantic-tokens 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-qualified-tys $(tests_dir)/analyzer/qualified-tys 2>&1 | python scripts/filter_rust_backtrace.py

# debugger

test-debugger:
	@cargo check -q
	cargo run --bin husky-debugger test tests/debugger

test-debugger-backtraced:
	cargo check
	RUST_BACKTRACE=1 cargo run --bin husky-debugger test tests/debugger

test-debugger-backtraced-filtered:
	cargo check
	RUST_BACKTRACE=1 cargo run --bin husky-debugger test tests/debugger 2>&1 | python scripts/filter_rust_backtrace.py

# compiler

test-compiler:
	cargo run --bin husky-compiler tests/compiler
	cargo run --bin husky-debugger test tests/compiler

test-compiler-backtraced:
	RUST_BACKTRACE=1 scripts/test-compiler.sh