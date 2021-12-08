use common::*;

use husky_lang_db::helpers::node_ext::vis_eq;
use rustc_hash::FxHashSet;

use syntax::{
    ast, Direction, NodeOrToken, SingleFileParseTree,
    SyntaxKind::{self, *},
};

use std::hash::Hash;

const REGION_START: &str = "// region:";
const REGION_END: &str = "// endregion";

#[derive(Debug, PartialEq, Eq)]
pub enum FoldKind {
    Comment,
    Imports,
    Mods,
    Block,
    ArgList,
    Region,
    Consts,
    Statics,
    Array,
    WhereClause,
    ReturnType,
}

#[derive(Debug)]
pub struct Fold {
    pub range: TextRange,
    pub kind: FoldKind,
}

// Feature: Folding
//
// Defines folding regions for curly braced blocks, runs of consecutive use, mod, const or static
// items, and `region` / `endregion` comment markers.
pub(crate) fn folding_ranges(file: &SingleFileParseTree) -> Vec<Fold> {
    todo!()
}

fn fold_kind(kind: SyntaxKind) -> Option<FoldKind> {
    todo!()
}

fn eq_visibility(vis0: Option<ast::Visibility>, vis1: Option<ast::Visibility>) -> bool {
    match (vis0, vis1) {
        (None, None) => true,
        (Some(vis0), Some(vis1)) => vis_eq(&vis0, &vis1),
        _ => false,
    }
}

fn contiguous_range_for_comment(
    first: ast::Comment,
    visited: &mut FxHashSet<ast::Comment>,
) -> Option<TextRange> {
    todo!()
}

fn fold_range_for_where_clause(where_clause: ast::WhereClause) -> Option<TextRange> {
    todo!()
}
