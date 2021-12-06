//! Handles the `Enter` key press. At the momently, this only continues
//! comments, but should handle indent some time in the future as well.

use common::*;

use ide_db::base_db::{FilePosition, SourceDatabase};
use ide_db::IdeDatabase;
use syntax::{
    ast::{self, edit::IndentLevel},
    SingleFileParseTree, SmolStr,
    SyntaxKind::*,
    SyntaxNode, SyntaxToken, TokenAtOffset,
};

use text_edit::TextEdit;

// Feature: On Enter
//
// husky-lang-server can override kbd:[Enter] key to make it smarter:
//
// - kbd:[Enter] inside triple-slash comments automatically inserts `///`
// - kbd:[Enter] in the middle or after a trailing space in `//` inserts `//`
// - kbd:[Enter] inside `//!` doc comments automatically inserts `//!`
// - kbd:[Enter] after `{` indents contents and closing `}` of single-line block
//
// This action needs to be assigned to shortcut explicitly.
//
// VS Code::
//
// Add the following to `keybindings.json`:
// [source,json]
// ----
// {
//   "key": "Enter",
//   "command": "husky-lang-server.onEnter",
//   "when": "editorTextFocus && !suggestWidgetVisible && editorLangId == rust"
// }
// ----
//
// When using the Vim plugin:
// [source,json]
// ----
// {
//   "key": "Enter",
//   "command": "husky-lang-server.onEnter",
//   "when": "editorTextFocus && !suggestWidgetVisible && editorLangId == rust && vim.mode == 'Insert'"
// }
// ----
//
// image::https://user-images.githubusercontent.com/48062697/113065578-04c21800-91b1-11eb-82b8-22b8c481e645.gif[]
pub(crate) fn on_enter(db: &IdeDatabase, position: FilePosition) -> Option<TextEdit> {
    todo!()
}

fn on_enter_in_comment(
    comment: &ast::Comment,
    file: &ast::SingleFileParseTree,
    offset: TextSize,
) -> Option<TextEdit> {
    todo!()
}

fn on_enter_in_block(block: ast::BlockExpr, position: FilePosition) -> Option<TextEdit> {
    todo!()
}

fn on_enter_in_use_tree_list(list: ast::UseTreeList, position: FilePosition) -> Option<TextEdit> {
    todo!()
}

fn block_contents(block: &ast::BlockExpr) -> Option<SyntaxNode> {
    todo!()
}

fn followed_by_comment(comment: &ast::Comment) -> bool {
    todo!()
}

fn node_indent(file: &SingleFileParseTree, token: &SyntaxToken) -> Option<SmolStr> {
    todo!()
}
