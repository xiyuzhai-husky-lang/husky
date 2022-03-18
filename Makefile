runtime_tests_dir=projects/tests/runtime
compile_time_tests_dir=projects/tests/compile-time

tr:
	cargo run --bin husky-lang-debugger $(runtime_tests_dir) --input-id 1 --mode test-runtime

trb:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(runtime_tests_dir) --input-id 1 --mode test-runtime

tc:
	cargo run --bin husky-lang-debugger $(compile_time_tests_dir) --input-id 1 --mode test-compile-time