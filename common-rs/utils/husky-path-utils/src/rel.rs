use pathdiff::diff_paths;

use crate::*;

pub fn get_relative_path(path: &Path) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    diff_paths(path, current_dir).unwrap()
}
