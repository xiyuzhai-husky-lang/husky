use pathdiff::diff_paths;

use crate::*;

pub fn get_path_relative_to_current(path: &Path) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    diff_paths(path, current_dir).unwrap()
}
