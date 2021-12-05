use std::iter::successors;

use hir::Semantics;
use ide_db::RootDatabase;
use syntax::{
    ast, Direction, NodeOrToken,
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, TextRange, TextSize, TokenAtOffset,
};

use crate::FileRange;

// Feature: Expand and Shrink Selection
//
// Extends or shrinks the current selection to the encompassing syntactic construct
// (expression, statement, item, module, etc). It works with multiple cursors.
//
// This is a standard LSP feature and not a protocol extension.
//
// |===
// | Editor  | Shortcut
//
// | VS Code | kbd:[Alt+Shift+→], kbd:[Alt+Shift+←]
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113020651-b42fc800-917a-11eb-8a4f-cf1a07859fac.gif[]
pub(crate) fn extend_selection(db: &RootDatabase, frange: FileRange) -> TextRange {
    todo!()
}

fn try_extend_selection(
    sema: &Semantics<RootDatabase>,
    root: &SyntaxNode,
    frange: FileRange,
) -> Option<TextRange> {
    todo!()
}

/// Find the shallowest node with same range, which allows us to traverse siblings.
fn shallowest_node(node: &SyntaxNode) -> SyntaxNode {
    todo!()
}

fn extend_single_word_in_comment_or_string(
    leaf: &SyntaxToken,
    offset: TextSize,
) -> Option<TextRange> {
    todo!()
}

fn extend_ws(root: &SyntaxNode, ws: SyntaxToken, offset: TextSize) -> TextRange {
    todo!()
}

fn pick_best(l: SyntaxToken, r: SyntaxToken) -> SyntaxToken {
    todo!()
}

/// Extend list item selection to include nearby delimiter and whitespace.
fn extend_list_item(node: &SyntaxNode) -> Option<TextRange> {
    todo!()
}

fn extend_comments(comment: ast::Comment) -> Option<TextRange> {
    todo!()
}

fn adj_comments(comment: &ast::Comment, dir: Direction) -> ast::Comment {
    todo!()
}
