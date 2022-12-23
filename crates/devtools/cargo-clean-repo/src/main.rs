use husky_path_utils::{clear_directory, collect_paths, collect_rust_package_dirs, Path};
use husky_print_utils::p;
use std::path::PathBuf;

fn main() {
    // make sure that this is the repository  https://github.com/xiyuzhai-husky-lang/husky
    assert!(PathBuf::from("Cargo.toml").exists());
    assert!(PathBuf::from("crates").exists());
    assert!(PathBuf::from("rust-toolchain").exists());
    assert!(PathBuf::from("husky-toolchain.toml").exists());
    assert!(PathBuf::from(".corgi/config.toml").exists());
    // clean_expect_files();
    clean_tests()
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

fn clean_tests() {
    let collect_paths = collect_paths(&PathBuf::from("tests"));
    for path in collect_paths {
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            std::fs::remove_file(path).unwrap()
        }
        // if path.file_name().and_then(|s| s.to_str()) == Some("__rust_gen__") {
        //     let new_path = path.with_file_name("rust");
        //     std::fs::rename(path, new_path).unwrap()
        // }
    }
}
