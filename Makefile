debugger_tests_dir=projects/tests/debugger

test:
	cargo run --bin husky-lang-debugger $(debugger_tests_dir) --input-id 1 --mode test
test_with_backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(debugger_tests_dir) --input-id 1 --mode test
