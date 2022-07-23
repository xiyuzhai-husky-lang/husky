mod module_tree;
mod rel;

use husky_check_utils::should_satisfy;
use husky_print_utils::p;
pub use module_tree::*;
pub use rel::*;

use std::path::{Path, PathBuf};

pub fn path_has_file_name(path: &Path, name: &str) -> bool {
    path.file_name().map(|s| s.to_string_lossy()) == Some(name.into())
}

pub fn path_file_name_str(path: &Path) -> Option<String> {
    path.file_name().map(|s| s.to_string_lossy().to_string())
}

pub fn path_parent_file_name_str(path: &Path) -> Option<String> {
    if let Some(parent) = path.parent() {
        parent.file_name().map(|s| s.to_string_lossy().to_string())
    } else {
        None
    }
}

pub fn path_has_extension(path: &Path, extension: &str) -> bool {
    path.extension().map(|s| s.to_string_lossy()) == Some(extension.into())
}

pub fn collect_all_package_dirs(dir: &Path) -> Vec<PathBuf> {
    should_satisfy!(dir, |dir: &Path| dir.is_dir());
    let main_path = dir.join("main.hsk");
    if main_path.exists() {
        return vec![dir.to_path_buf()];
    } else {
        let mut pack_paths = vec![];
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let subpath = entry.path();
            if subpath.is_dir() {
                pack_paths.extend(collect_all_package_dirs(&subpath))
            }
        }
        pack_paths
    }
}

pub fn collect_all_source_files(dir: PathBuf) -> Vec<PathBuf> {
    assert!(dir.is_dir());
    let mut source_files = vec![];
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if subpath.is_dir() {
            source_files.extend(collect_all_source_files(subpath))
        } else {
            if subpath.extension().unwrap() == "hsk" {
                source_files.push(subpath)
            }
        }
    }
    source_files
}
