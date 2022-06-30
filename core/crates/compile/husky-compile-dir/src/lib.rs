use pack_semantics::Package;
use print_utils::p;
use std::{
    env,
    path::{Path, PathBuf},
};
use word::snake_to_dash;

pub fn get_rust_dir(pack: &Package) -> PathBuf {
    let husky_dir = env::var("HUSKY_DIR").unwrap();
    let rust_root = format!("{husky_dir}/.compiled/crates");
    let rust_dir: PathBuf = [rust_root, snake_to_dash(&pack.ident)].iter().collect();
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

pub fn get_code_snapshot_dir(pack: &Package) -> String {
    let rust_dir = get_rust_dir(pack);
    assert!(rust_dir.exists());
    let snapshot_dir = rust_dir.join("snapshot");
    assert!(snapshot_dir.exists());
    snapshot_dir.to_str().unwrap().into()
}
