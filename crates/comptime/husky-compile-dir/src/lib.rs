use std::path::{Path, PathBuf};

pub fn getx_child_dir(parent_dir: &Path, dirname: &str) -> PathBuf {
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
