use hir::db::DefDatabase;
use husky_lang_db::vfs::SourceFileId;
use husky_lang_db::HuskyLangDatabase;

// Feature: Debug ItemTree
//
// Displays the ItemTree of the currently open file, for debugging.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Debug ItemTree**
// |===
pub(crate) fn view_item_tree(db: &HuskyLangDatabase, file_id: SourceFileId) -> String {
    todo!()
}
