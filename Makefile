projects_dir=/home/xiyuzhai/Documents/husky/projects
examples_dir=/home/xiyuzhai/Documents/husky/examples
temp_dir=/home/xiyuzhai/Documents/husky/examples/good/input

test-examples:
	cargo run --bin husky-lang-compiler $(examples_dir)/good
	cargo run --bin husky-lang-debugger $(examples_dir) --input-id 1 --mode test

test-examples-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(examples_dir) --input-id 1 --mode test 2>&1 | python scripts/filter_rust_backtrace.py

test-temp:
	cargo run --bin husky-lang-compiler $(temp_dir)
	cargo run --bin husky-lang-debugger $(temp_dir) --input-id 1 --mode test

#test-compile-time:
#	cargo run --bin husky-lang-debugger $(compile_time_tests_dir) --input-id 1 --mode test-compile-time
#
#test-compiler:
#	cargo run --bin husky-lang-compiler $(runtime_tests_dir)
#
#test-compiler-with-backtrace:
#	RUST_BACKTRACE=1 cargo run --bin husky-lang-compiler $(runtime_tests_dir)

mnist:
	cargo run --bin husky-lang-debugger $(projects_dir)/cv/mnist-classifier --input-id 11 --mode run

mnist-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(projects_dir)/cv/mnist-classifier --input-id 11 --mode run 2>&1 | python scripts/filter_rust_backtrace.py

install:
	scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer ~/.vscode/extensions/
	cargo install --path crates/apps/analyzer
	cargo install --path crates/apps/compiler
	cargo install --path crates/apps/debugger
