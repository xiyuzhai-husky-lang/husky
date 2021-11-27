use semantics::db::DefDatabase;
use ide_db::base_db::FileID;
use ide_db::RootDatabase;

// Feature: Debug ItemTree
//
// Displays the ItemTree of the currently open file, for debugging.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Debug ItemTree**
// |===
pub(crate) fn view_item_tree(db: &RootDatabase, file_id: FileID) -> String {
    db.file_item_tree(file_id.into()).pretty_print()
}
