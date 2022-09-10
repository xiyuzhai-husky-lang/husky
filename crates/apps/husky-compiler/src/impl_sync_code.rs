use husky_io_utils::{
    file_sync::diff_file_sync, relative_path_pattern::RelativePathPattern, FileVisitConfig,
};

use crate::*;

impl CompilerInstance {
    pub(crate) fn sync_rust_code(&self, package_dir: &Path, verbose: bool) {
        diff_file_sync(
            package_dir.join("__rust_gen_cache__"),
            package_dir.join("__rust_gen__"),
            FileVisitConfig {
                regular_file_filter: RelativePathPattern::extension_is_among(["toml", "hsy", "rs"]),
                dir_filter: RelativePathPattern::ignore_paths(["target"]),
                verbose,
            },
        )
    }
}
