use common::*;

use husky_lang_db::HuskyLangDatabase;
use syntax::{NodeOrToken, SingleFileParseTree, SyntaxToken};
use vfs::{FileId, VirtualFileSystem};

// Feature: Show Syntax Tree
//
// Shows the parse tree of the current file. It exists mostly for debugging
// husky-lang-server itself.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Show Syntax Tree**
// |===
// image::https://user-images.githubusercontent.com/48062697/113065586-068bdb80-91b1-11eb-9507-fee67f9f45a0.gif[]
pub(crate) fn syntax_tree(
    db: &HuskyLangDatabase,
    file_id: FileId,
    text_range: Option<TextRange>,
) -> String {
    todo!()
}

/// Attempts parsing the selected contents of a string literal
/// as rust syntax and returns its syntax tree
fn syntax_tree_for_string(token: &SyntaxToken, text_range: TextRange) -> Option<String> {
    todo!()
}

fn syntax_tree_for_token(node: &SyntaxToken, text_range: TextRange) -> Option<String> {
    todo!()
}
