examples_dir=examples
tests_dir=tests

include tests/makefile
include examples/makefile

update-python-requirements:
	pipreqs ./ --force

# ad hoc
install-toolchain:
	mkdir -p ~/.huskyup/toolchains/nightly/lib/rustlib/src/husky/library
	rsync -a library ~/.huskyup/toolchains/nightly/lib/rustlib/src/husky/

vscode: install-toolchain
	# scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer/language-configuration.json ~/.vscode/extensions/husky-lang.husky-analyzer-0.1.0/language-configuration.json
	cargo install --path crates/apps/husky-analyzer --bin husky-analyzer-server
	spd-say "vscode is done"

vscode-server: install-toolchain
	# scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer/language-configuration.json ~/.vscode-server/extensions/husky-lang.husky-analyzer-0.1.0/language-configuration.json
	cargo install --path crates/apps/husky-analyzer --bin husky-analyzer-server
	# spd-say "vscode is done"

install-compiler:
	cargo install --path crates/apps/husky-compiler --bin husky-compiler

count-todo:
	scripts/pattern_statistics.py "todo!()" crates 1 10
	scripts/pattern_statistics.py "todo!()" crates 2 10

update-expect:
	UPDATE_EXPECT=1 cargo test --features "allow-print" -- --test-threads 1 --nocapture\
		|| scripts/play_update_expect_failure_music.sh
	scripts/play_update_expect_success_music.sh

update-expect-server:
	UPDATE_EXPECT=1 cargo test --features "allow-print" -- --test-threads 1 --nocapture

ubuntu-setup:
	scripts/ubuntu_setup.sh

test-digitize:
	cargo run --bin digitize -- data/typical-huskies0/n02109961_57.JPEG

test-digitize-ultraman:
	cargo run --bin digitize -- data/ultraman/leo/images.jpeg

install-devtools:
	cargo install --path crates/devtools/cargo-organise

organise: install-devtools
	cargo organise

adversarial:
	# cargo test
	ADVERSARIAL_ROUND=1000 cargo test

run-notebook:
	cargo run --path crates/apps/husky-notebook
