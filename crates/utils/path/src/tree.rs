use print_utils::p;

use crate::*;

macro_rules! try_path {
    ($path: expr) => {{
        if $path.exists() {
            return $path;
        }
    }};
}

pub fn parent_module_path(path: &Path) -> PathBuf {
    try_path!(path.with_file_name("main.hsk"));
    try_path!(path.with_file_name("mod.hsk"));
    try_path!(path.parent().unwrap().with_extension("hsk"));
    todo!()
}
