# cargo

cargo-test:
	cargo test

cargo-test-backtraced:
	RUST_BACKTRACE=1 cargo test

# analyzer

test-analyzer:
	cargo run --bin husky-analyzer-tester test-diagnostics $(tests_dir)/diagnostics \
	&& cargo run --bin husky-analyzer-tester test-folding-ranges $(tests_dir)/folding-ranges \
	&& cargo run --bin husky-analyzer-tester test-semantic-tokens $(tests_dir)/semantic-tokens\
	&& cargo run --bin husky-analyzer-tester test-qualified-tys $(tests_dir)/qualified-tys

test-analyzer-backtraced:
	set -e
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-diagnostics $(tests_dir)/diagnostics
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-folding-ranges $(tests_dir)/folding-ranges
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-semantic-tokens $(tests_dir)/semantic-tokens
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-qualified-tys $(tests_dir)/qualified-tys

test-analyzer-backtraced-filtered:
	set -e
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-diagnostics $(tests_dir)/diagnostics 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-folding-ranges $(tests_dir)/folding-ranges 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-semantic-tokens $(tests_dir)/semantic-tokens 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run --bin husky-analyzer-tester test-qualified-tys $(tests_dir)/qualified-tys 2>&1 | python scripts/filter_rust_backtrace.py

add-mnist-classifier-to-diagnostics-tests:
	cargo run -q --bin husky-test-add examples/mnist-classifier diagnostics misc

add-mnist-classifier-to-folding-ranges-tests:
	cargo run -q --bin husky-test-add examples/mnist-classifier folding-ranges misc

add-mnist-classifier-to-qualified-tys-tests:
	cargo run -q --bin husky-test-add examples/mnist-classifier qualified-tys misc

test-dev:
	@cargo check -q
	cargo run --bin husky-dev test tests/dev

test-dev-backtraced:
	cargo check
	RUST_BACKTRACE=1 cargo run --bin husky-dev test tests/dev

test-dev-backtraced-filtered:
	cargo check
	RUST_BACKTRACE=1 cargo run --bin husky-dev test tests/dev 2>&1 | python scripts/filter_rust_backtrace.py

# compiler

test-compiler:
	cargo run --bin husky-compiler tests/compiler
	cargo run --bin husky-dev test tests/compiler

test-compiler-backtraced:
	RUST_BACKTRACE=1 cargo run --bin husky-compiler tests/compiler
	RUST_BACKTRACE=1 cargo run --bin husky-dev test tests/compiler

test-compiler-backtraced-filtered:
	RUST_BACKTRACE=1 cargo run --bin husky-compiler tests/compiler 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run --bin husky-dev test tests/compiler 2>&1 | python scripts/filter_rust_backtrace.py

test-update-dirnames:
	cargo run --bin husky-test-update-dirname