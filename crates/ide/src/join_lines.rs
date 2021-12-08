use common::*;

use ide_assists::utils::extract_trivial_expression;
use husky_lang_db::helpers::node_ext::expr_as_name_ref;
use itertools::Itertools;
use syntax::{ast, NodeOrToken, SingleFileParseTree, SyntaxElement, SyntaxKind, SyntaxToken};

use text_edit::{TextEdit, TextEditBuilder};

pub struct JoinLinesConfig {
    pub join_else_if: bool,
    pub remove_trailing_comma: bool,
    pub unwrap_trivial_blocks: bool,
    pub join_assignments: bool,
}

// Feature: Join Lines
//
// Join selected lines into one, smartly fixing up whitespace, trailing commas, and braces.
//
// See
// https://user-images.githubusercontent.com/1711539/124515923-4504e800-dde9-11eb-8d58-d97945a1a785.gif[this gif]
// for the cases handled specially by joined lines.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Join lines**
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113020661-b6922200-917a-11eb-87c4-b75acc028f11.gif[]
pub(crate) fn join_lines(
    config: &JoinLinesConfig,
    file: &SingleFileParseTree,
    range: TextRange,
) -> TextEdit {
    todo!()
}

fn remove_newlines(
    config: &JoinLinesConfig,
    edit: &mut TextEditBuilder,
    token: &SyntaxToken,
    range: TextRange,
) {
    todo!()
}

fn remove_newline(
    config: &JoinLinesConfig,
    edit: &mut TextEditBuilder,
    token: &SyntaxToken,
    offset: TextSize,
) {
    todo!()
}

fn join_single_expr_block(edit: &mut TextEditBuilder, token: &SyntaxToken) -> Option<()> {
    todo!()
}

fn join_single_use_tree(edit: &mut TextEditBuilder, token: &SyntaxToken) -> Option<()> {
    todo!()
}

fn join_assignments(
    edit: &mut TextEditBuilder,
    prev: &SyntaxElement,
    next: &SyntaxElement,
) -> Option<()> {
    todo!()
}

fn as_if_expr(element: &SyntaxElement) -> Option<ast::IfExpr> {
    todo!()
}

fn compute_ws(left: SyntaxKind, right: SyntaxKind) -> &'static str {
    todo!()
}
