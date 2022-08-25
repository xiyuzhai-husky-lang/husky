use husky_io_utils::{
    diff_copy, file_sync::diff_file_sync, path_pattern::PathPattern, FileVisitConfig,
};

use crate::*;
use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    process::Command,
};

impl CompilerInstance {
    pub(crate) fn sync_rust_code(&self, package_dir: &Path) {
        diff_file_sync(
            package_dir.join("__rust_gen_cache__"),
            package_dir.join("__rust_gen__"),
            &FileVisitConfig {
                regular_file_filter: PathPattern::extension_is_among(["toml", "hsk", "rs"]),
                dir_filter: PathPattern::ignore_paths(package_dir, ["target"]),
            },
        )
    }
}
// ["toml", "hsk", "rs"],
//             &["target", "Cargo.lock"],
//             &[],
