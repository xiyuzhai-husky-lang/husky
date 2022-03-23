runtime_tests_dir=projects/tests/runtime
compile_time_tests_dir=projects/tests/compile-time

test-runtime:
	cargo run --bin husky-lang-debugger $(runtime_tests_dir) --input-id 1 --mode test-runtime
	cargo run --bin husky-lang-debugger $(runtime_tests_dir) --input-id 1 --mode test-runtime -c

test-runtime-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(runtime_tests_dir) --input-id 1 --mode test-runtime

test-compile-time:
	cargo run --bin husky-lang-debugger $(compile_time_tests_dir) --input-id 1 --mode test-compile-time
	cargo run --bin husky-lang-debugger $(compile_time_tests_dir) --input-id 1 --mode test-compile-time -c
