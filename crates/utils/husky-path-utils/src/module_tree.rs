use husky_check_utils::should_eq;
use husky_print_utils::p;

use crate::*;

macro_rules! try_path {
    ($path: expr, $exists: expr) => {{
        if $exists(&$path) {
            return Some($path);
        }
    }};
}

pub fn parent_module_path(path: &Path, exists: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    try_path!(path.with_file_name("main.hsy"), exists);
    try_path!(path.parent().unwrap().with_extension("hsy"), exists);
    todo!()
}

pub fn submodule_path(path: &Path, ident: &str, exists: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    should_eq!(path.extension().unwrap(), "hsy");
    if path.file_name().unwrap() == "main.hsy" {
        submodule_path2(path.parent().unwrap(), ident, exists)
    } else if path.extension().unwrap() == "hsy" {
        submodule_path2(&path.with_extension(""), ident, exists)
    } else {
        p!(path);
        todo!()
    }
}

fn submodule_path2(dir: &Path, ident: &str, exists: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    let maybe_result = dir.join(format!("{}.hsy", ident));
    if exists(&maybe_result) {
        return Some(maybe_result);
    } else {
        None
    }
}
