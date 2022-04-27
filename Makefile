projects_dir=/home/xiyuzhai/Documents/husky/projects
examples_dir=/home/xiyuzhai/Documents/husky/examples

test-examples:
	cargo run --bin husky-lang-debugger $(examples_dir) --input-id 1 --mode test
	cargo run --bin husky-lang-compiler $(examples_dir)/good

test-examples-with-backtrace:
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(examples_dir) --input-id 1 --mode test

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
	RUST_BACKTRACE=1 cargo run --bin husky-lang-debugger $(projects_dir)/cv/mnist-classifier --input-id 11 --mode test-runtime 2>&1 | python scripts/filter_rust_backtrace.py

install:
	scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer ~/.vscode/extensions/
	cargo install --path crates/apps/analyzer
	cargo install --path crates/apps/compiler
	cargo install --path crates/apps/debugger
