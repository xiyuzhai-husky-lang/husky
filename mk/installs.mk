vscode: cargo-test test-analyzer
	scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer ~/.vscode/extensions/
	cargo install --path crates/apps/husky-analyzer --bin husky-analyzer-server

install-compiler:
	cargo install --path crates/apps/husky-compiler --bin husky-compiler
