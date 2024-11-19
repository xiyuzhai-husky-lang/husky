use super::*;
use helpers::tracker::VdLeanTranspilationTracker;
use latex_prelude::{
    helper::tracker::{LxDocumentBodyInput, LxDocumentInput, LxPageInput},
    mode::LxMode,
};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;

fn t(content: &str, expected_display_tree: &Expect, expected_fmt: &Expect) {
    let db = &DB::default();
    let file_path = LxFilePath::new(db, PathBuf::from(file!()));
    let tracker =
        VdLeanTranspilationTracker::new(LxDocumentInput { file_path, content }, &[], &[], db);
    expected_display_tree.assert_eq(&tracker.show_display_tree(db));
    expected_fmt.assert_eq(&tracker.show_fmt(db));
}
