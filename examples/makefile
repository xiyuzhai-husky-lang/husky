
# mnist

mnist:
	cargo check
	cargo run --bin husky-developer -- --package-dir $(examples_dir)/mnist-classifier --sample-id 23 --mode run

mnist-with-backtrace:
	cargo check
	RUST_BACKTRACE=1 cargo run --bin husky-developer -- --package-dir $(examples_dir)/mnist-classifier --sample-id 23 --mode run

mnist-with-backtrace-filtered:
	RUST_BACKTRACE=1 cargo run --bin husky-developer -- --package-dir $(examples_dir)/mnist-classifier --sample-id 23 --mode run 2>&1 | python scripts/filter_rust_backtrace.py

mnist-release:
	cargo check
	cargo install --path crates/apps/husky-developer --bin husky-developer
	husky-developer --package-dir $(examples_dir)/mnist-classifier --sample-id 23 --mode run

mnist-compiled:
	cargo check
	cargo run --bin husky-compiler -- $(examples_dir)/mnist-classifier
	cargo run --bin husky-developer -- run $(examples_dir)/mnist-classifier

mnist-compiled-backtraced:
	cargo check
	RUST_BACKTRACE=1 cargo run --bin husky-compiler -- $(examples_dir)/mnist-classifier
	RUST_BACKTRACE=1 cargo run --bin husky-developer -- run $(examples_dir)/mnist-classifier

mnist-compiled-backtraced-filtered:
	cargo check
	RUST_BACKTRACE=1 cargo run --bin husky-compiler -- $(examples_dir)/mnist-classifier 2>&1 | python scripts/filter_rust_backtrace.py
	RUST_BACKTRACE=1 cargo run --bin husky-developer -- run $(examples_dir)/mnist-classifier 2>&1 | python scripts/filter_rust_backtrace.py

print-mnist-qualified-tys:
	cargo run --bin husky-analyzer-printer print-qualified-tys $(examples_dir)/mnist-classifier
