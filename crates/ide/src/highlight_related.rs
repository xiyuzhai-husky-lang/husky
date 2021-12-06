use common::*;

use hir::Semantics;
use ide_db::{
    base_db::{FileID, FilePosition},
    defs::Definition,
    helpers::{for_each_break_expr, for_each_tail_expr, node_ext::walk_expr, pick_best_token},
    search::{FileReference, ReferenceCategory, SearchScope},
    IdeDatabase,
};
use rustc_hash::FxHashSet;
use syntax::{ast, SyntaxNode, SyntaxToken};

use crate::{references, NavigationTarget, TryToNav};

#[derive(PartialEq, Eq, Hash)]
pub struct HighlightedRange {
    pub range: TextRange,
    // FIXME: This needs to be more precise. Reference category makes sense only
    // for references, but we also have defs. And things like exit points are
    // neither.
    pub category: Option<ReferenceCategory>,
}

#[derive(Default, Clone)]
pub struct HighlightRelatedConfig {
    pub references: bool,
    pub exit_points: bool,
    pub break_points: bool,
    pub yield_points: bool,
}

// Feature: Highlight Related
//
// Highlights constructs related to the thing under the cursor:
// - if on an identifier, highlights all references to that identifier in the current file
// - if on an `async` or `await token, highlights all yield points for that async context
// - if on a `return` or `fn` keyword, `?` character or `->` return type arrow, highlights all exit points for that context
// - if on a `break`, `loop`, `while` or `for` token, highlights all break points for that loop or block context
//
// Note: `?` and `->` do not currently trigger this behavior in the VSCode editor.
pub(crate) fn highlight_related(
    sema: &Semantics<IdeDatabase>,
    config: HighlightRelatedConfig,
    FilePosition { offset, file_id }: FilePosition,
) -> Option<Vec<HighlightedRange>> {
    todo!()
}

fn highlight_references(
    sema: &Semantics<IdeDatabase>,
    node: &SyntaxNode,
    token: SyntaxToken,
    file_id: FileID,
) -> Option<Vec<HighlightedRange>> {
    todo!()
}

fn highlight_exit_points(
    sema: &Semantics<IdeDatabase>,
    token: SyntaxToken,
) -> Option<Vec<HighlightedRange>> {
    todo!()
}

fn highlight_break_points(token: SyntaxToken) -> Option<Vec<HighlightedRange>> {
    todo!()
}

fn highlight_yield_points(token: SyntaxToken) -> Option<Vec<HighlightedRange>> {
    todo!()
}

fn cover_range(r0: Option<TextRange>, r1: Option<TextRange>) -> Option<TextRange> {
    match (r0, r1) {
        (Some(r0), Some(r1)) => Some(r0.cover(r1)),
        (Some(range), None) => Some(range),
        (None, Some(range)) => Some(range),
        (None, None) => None,
    }
}

fn find_defs(sema: &Semantics<IdeDatabase>, token: SyntaxToken) -> FxHashSet<Definition> {
    todo!()
}
