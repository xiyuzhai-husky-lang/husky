runtime_tests_dir=/home/xiyuzhai/Documents/husky/projects/tests/runtime
compile_time_tests_dir=/home/xiyuzhai/Documents/husky/projects/tests/compile-time

test-runtime:
	cargo run --bin husky-lang-debugger $(runtime_tests_dir) --input-id 1 --mode test-runtime
	#cargo run --bin husky-lang-debugger $(runtime_tests_dir) --input-id 1 --mode test-runtime -c

test-runtime-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(runtime_tests_dir) --input-id 1 --mode test-runtime

test-compile-time:
	cargo run --bin husky-lang-debugger $(compile_time_tests_dir) --input-id 1 --mode test-compile-time
	#cargo run --bin husky-lang-debugger $(compile_time_tests_dir) --input-id 1 --mode test-compile-time -c

test-compiler:
	cargo run --bin husky-lang-compiler $(runtime_tests_dir)

test-compiler-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-compiler $(runtime_tests_dir)
