use io_utils::{diff_copy, diff_sync};

use crate::*;
use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    process::Command,
};

impl CompilerInstance {
    pub(crate) fn sync_rust_code(&self, package_dir: &Path) {
        diff_sync(
            package_dir.join("__rust_gen_cache__"),
            package_dir.join("__rust_gen__"),
            &["toml", "hsk", "rs"],
            &["target", "Cargo.lock"],
            &[],
        )
    }
}
