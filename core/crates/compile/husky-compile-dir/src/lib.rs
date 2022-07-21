use husky_package_semantics::Package;
use print_utils::p;
use std::{
    env,
    path::{Path, PathBuf},
};
use word::snake_to_dash;

pub fn get_rust_dir(pack: &Package) -> PathBuf {
    let husky_dir = env::var("HUSKY_DIR").unwrap();
    let rust_root = format!("{husky_dir}/.compiled/crates");
    let dashed_name = snake_to_dash(&pack.ident);
    let rust_dir: PathBuf = [rust_root, dashed_name].iter().collect();
    mkdir(&rust_dir);
    rust_dir
}

pub fn get_or_create_child_dir(parent_dir: &Path, dirname: &str) -> PathBuf {
    let child_dir = parent_dir.join(dirname);
    mkdir(&child_dir);
    child_dir
}

pub fn mkdir(dir: &Path) {
    if !dir.exists() {
        std::fs::create_dir_all(&dir).unwrap();
    } else {
        if !dir.is_dir() {
            panic!()
        }
    }
}

pub fn get_husky_code_snapshot_dir(package: &Package) -> PathBuf {
    let rust_dir = get_rust_dir(package);
    assert!(rust_dir.exists());
    let snapshot_dir = get_or_create_child_dir(&rust_dir, "snapshot");
    get_or_create_child_dir(&snapshot_dir, &snake_to_dash(&package.ident))
}
