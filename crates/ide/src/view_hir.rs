use hir::{Function, Semantics};
use ide_db::base_db::FilePosition;
use ide_db::RootDatabase;
use syntax::ast;

// Feature: View Hir
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: View Hir**
// |===
// image::https://user-images.githubusercontent.com/48062697/113065588-068bdb80-91b1-11eb-9a78-0b4ef1e972fb.gif[]
pub(crate) fn view_hir(db: &RootDatabase, position: FilePosition) -> String {
    body_hir(db, position).unwrap_or_else(|| "Not inside a function body".to_string())
}

fn body_hir(db: &RootDatabase, position: FilePosition) -> Option<String> {
    todo!()
}
