
# mnist

mnist:
	cargo check
	cargo run -q --bin husky-debugger -- --package-dir $(projects_dir)/cv/mnist-classifier --sample-id 23 --mode run

mnist-release:
	cargo check
	cargo install --path crates/apps/husky-debugger --bin husky-debugger
	husky-debugger --package-dir $(projects_dir)/cv/mnist-classifier --sample-id 23 --mode run

mnist-compiled:
	cargo check
	cargo run -q --bin husky-compiler $(projects_dir)/cv/mnist-classifier
	cd ${HUSKY_DIR}/.compiled && cargo run -q mnist-classifier-debugger --warn-missing-linkage

mnist-compiled-release:
	cargo check
	cargo run -q --bin husky-compiler $(projects_dir)/cv/mnist-classifier
	cd ${HUSKY_DIR}/.compiled && cargo run -q --release mnist-classifier-debugger --warn-missing-linkage

mnist-compiled-with-backtrace:
	@cargo check
	@RUST_BACKTRACE=1 cargo run -q --bin husky-compiler $(projects_dir)/cv/mnist-classifier
	@cd ${HUSKY_DIR}/.compiled&& RUST_BACKTRACE=1 cargo run -q mnist-classifier-debugger

mnist-compiled-with-backtrace-filtered:
	@cargo check
	@RUST_BACKTRACE=1 cargo run -q --bin husky-compiler $(projects_dir)/cv/mnist-classifier 2>&1 | python ${HUSKY_DIR}/core/scripts/filter_rust_backtrace.py
	@cd ${HUSKY_DIR}/.compiled&& RUST_BACKTRACE=1 cargo run -q mnist-classifier-debugger 2>&1 | python ${HUSKY_DIR}/core/scripts/filter_rust_backtrace.py

print-mnist:
	cargo run -q --bin husky-analyzer-printer print-qualified-tys $(projects_dir)/cv/mnist-classifier

mnist-with-backtrace:
	cargo check
	RUST_BACKTRACE=1 cargo run -q --bin husky-debugger $(projects_dir)/cv/mnist-classifier --sample-id 23 --mode run

mnist-with-backtrace-filtered:
	RUST_BACKTRACE=1 cargo run -q --bin husky-debugger $(projects_dir)/cv/mnist-classifier --sample-id 23 --mode run 2>&1 | python scripts/filter_rust_backtrace.py
