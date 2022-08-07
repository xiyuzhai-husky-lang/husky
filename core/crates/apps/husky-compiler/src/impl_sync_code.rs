use io_utils::diff_copy;

use crate::*;
use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    process::Command,
};

impl CompilerInstance {
    pub(crate) fn sync_rust_code(&self, package_dir: &Path) {
        sync(
            package_dir.join("__rust_gen_cache__"),
            package_dir.join("__rust_gen__"),
            &["toml", "hsk", "rs"],
            &["target"],
        )
    }
}

fn sync(src: PathBuf, dst: PathBuf, extensions: &[&'static str], excludes: &[&'static str]) {
    assert!(src.exists());
    if !dst.exists() {
        if src.is_dir() {
            fs::create_dir(dst.clone()).unwrap();
        }
    }
    if src.is_file() {
        assert!(extensions.contains(&src.extension().unwrap().to_str().unwrap()));
        if dst.is_dir() {
            fs::remove_dir_all(dst.clone()).unwrap()
        }
        diff_copy(&src, &dst);
    } else {
        let src_entries: HashMap<String, PathBuf> = fs::read_dir(src)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (path.file_name().unwrap().to_str().unwrap().to_owned(), path)
            })
            .collect();
        let dst_entries: HashMap<String, PathBuf> = fs::read_dir(dst.clone())
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (path.file_name().unwrap().to_str().unwrap().to_owned(), path)
            })
            .collect();
        for (filename, dst_entry) in dst_entries {
            if !src_entries.contains_key(&filename) {
                todo!("delete it")
            }
        }
        for (filename, src_entry) in src_entries {
            let dst_entry = dst.join(filename);
            sync(src_entry, dst_entry, extensions, &[])
        }
    }
}
