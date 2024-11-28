use crate::FILE_STORAGE;
use husky_path_utils::HuskyLangDevPaths;
use latex_ast::helpers::tracker::LxAstTracker;
use latex_prelude::helper::tracker::LxDocumentInput;
use latex_vfs::path::LxFilePath;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref TRACKERS: Vec<LxAstTracker<'static, LxDocumentInput<'static>>> = trackers();
}

// TODO: maybe use memo-like abstraction for this?
fn trackers() -> Vec<LxAstTracker<'static, LxDocumentInput<'static>>> {
    let dev_paths = HuskyLangDevPaths::new();
    let specs_dir = dev_paths.specs_dir();
    FILE_STORAGE
        .files()
        .iter()
        .enumerate()
        .map(|(i, content)| {
            let file_path = LxFilePath::new(PathBuf::from(format!("lx-file-{i}",)));
            LxAstTracker::new(LxDocumentInput {
                specs_dir: specs_dir.to_path_buf(),
                file_path,
                content,
            })
        })
        .collect()
}

#[test]
fn trackers_works() {
    let trackers = trackers();
    assert_eq!(trackers.len(), FILE_STORAGE.files().len());
}
