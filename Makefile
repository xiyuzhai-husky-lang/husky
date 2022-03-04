current_test_project=projects/tests/current

run:
	cargo run --bin husky-lang-debugger $(current_test_project) --input-id 1
run_with_backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(current_test_project) --input-id 1
