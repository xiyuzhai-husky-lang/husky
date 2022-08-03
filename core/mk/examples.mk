
# mnist

mnist:
	cargo check
	cargo run --bin husky-debugger -- --package-dir $(examples_dir)/cv/mnist-classifier --sample-id 23 --mode run

mnist-with-backtrace:
	cargo check
	RUST_BACKTRACE=1 cargo run --bin husky-debugger -- --package-dir $(examples_dir)/cv/mnist-classifier --sample-id 23 --mode run

mnist-with-backtrace-filtered:
	RUST_BACKTRACE=1 cargo run --bin husky-debugger -- --package-dir $(examples_dir)/cv/mnist-classifier --sample-id 23 --mode run 2>&1 | python scripts/filter_rust_backtrace.py

mnist-release:
	cargo check
	cargo install --path crates/apps/husky-debugger --bin husky-debugger
	husky-debugger --package-dir $(examples_dir)/cv/mnist-classifier --sample-id 23 --mode run

mnist-compiled:
	cargo check
	cargo run --bin husky-compiler $(examples_dir)/cv/mnist-classifier
	cd $(examples_dir)/cv/mnist-classifier/__rust_gen_cache__ && cargo build --release
	cargo run --bin husky-debugger -- --package-dir $(examples_dir)/cv/mnist-classifier --cdylib $(examples_dir)/cv/mnist-classifier/__rust_gen_cache__/target/release/libmnist_classifier.so --compiled

mnist-compiled-release:
	cargo check
	cargo run --bin husky-compiler $(examples_dir)/cv/mnist-classifier
	cd ${HUSKY_DIR}/__rust_gen__ && cargo run --release mnist-classifier-debugger --warn-missing-linkage

mnist-compiled-with-backtrace:
	@cargo check
	@RUST_BACKTRACE=1 cargo run --bin husky-compiler $(examples_dir)/cv/mnist-classifier
	@cd ${HUSKY_DIR}/__rust_gen__&& RUST_BACKTRACE=1 cargo run mnist-classifier-debugger

mnist-compiled-with-backtrace-filtered:
	@cargo check
	@RUST_BACKTRACE=1 cargo run --bin husky-compiler $(examples_dir)/cv/mnist-classifier 2>&1 | python ${HUSKY_DIR}/core/scripts/filter_rust_backtrace.py
	@cd ${HUSKY_DIR}/__rust_gen__&& RUST_BACKTRACE=1 cargo run mnist-classifier-debugger 2>&1 | python ${HUSKY_DIR}/core/scripts/filter_rust_backtrace.py

print-mnist:
	cargo run --bin husky-analyzer-printer print-qualified-tys $(examples_dir)/cv/mnist-classifier
