use hir::AsAssocItem;
use ide_db::helpers::{
    insert_use::{insert_use, ImportScope},
    mod_path_to_ast,
};
use syntax::{ast, SyntaxNode};

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: replace_qualified_name_with_use
//
// Adds a use statement for a given fully-qualified name.
//
// ```
// # mod std { pub mod collections { pub struct HashMap<T, U>(T, U); } }
// fn process(map: std::collections::$0HashMap<String, String>) {}
// ```
// ->
// ```
// use std::collections::HashMap;
//
// # mod std { pub mod collections { pub struct HashMap<T, U>(T, U); } }
// fn process(map: HashMap<String, String>) {}
// ```
pub(crate) fn replace_qualified_name_with_use(
    acc: &mut Assists,
    ctx: &AssistContext,
) -> Option<()> {
    todo!()
}

/// Adds replacements to `re` that shorten `path` in all descendants of `node`.
fn shorten_paths(node: &SyntaxNode, path: &ast::Path) {
    todo!()
}

fn maybe_replace_path(path: ast::Path, target: ast::Path) -> Option<()> {
    todo!()
}

fn path_eq_no_generics(lhs: ast::Path, rhs: ast::Path) -> bool {
    todo!()
}
