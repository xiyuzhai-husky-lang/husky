use husky_path_utils::{clear_directory, collect_rust_package_dirs, Path};
use std::path::PathBuf;

fn main() {
    // make sure that this is the repository  https://github.com/xiyuzhai-husky-lang/husky
    assert!(PathBuf::from("Cargo.toml").exists());
    assert!(PathBuf::from("crates").exists());
    assert!(PathBuf::from("rust-toolchain").exists());
    assert!(PathBuf::from("husky-toolchain.toml").exists());
    assert!(PathBuf::from(".corgi/config.toml").exists());
    clean_expect_files()
}

fn clean_expect_files() {
    for dir in collect_rust_package_dirs(".") {
        assert!(dir.join("Cargo.toml").exists());
        let expect_files_dir = dir.join("expect-files");
        if expect_files_dir.exists() {
            assert!(expect_files_dir.is_dir());
        }
        clear_directory(&expect_files_dir);
    }
}
