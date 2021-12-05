//! A module with ide helpers for high-level ide features.
pub mod famous_defs;
pub mod generated_lints;
pub mod import_assets;
pub mod insert_use;
pub mod merge_imports;
pub mod node_ext;
pub mod rust_doc;

use std::{collections::VecDeque, iter};

use base_db::FileID;
use hir::{ItemInNs, MacroDef, ModuleDef, Name, PathResolution, Semantics};
use itertools::Itertools;
use syntax::{
    ast::{self, Ident},
    Direction, SyntaxElement, SyntaxKind, SyntaxToken, TokenAtOffset, WalkEvent,
};

use crate::{defs::Definition, RootDatabase};

pub use self::famous_defs::FamousDefs;

pub fn item_name(db: &RootDatabase, item: ItemInNs) -> Option<Name> {
    todo!()
}

/// Parses and returns the derive path at the cursor position in the given attribute, if it is a derive.
/// This special case is required because the derive macro is a compiler builtin that discards the input derives.
///
/// The returned path is synthesized from TokenTree tokens and as such cannot be used with the [`Semantics`].
pub fn get_path_in_derive_attr(
    sema: &hir::Semantics<RootDatabase>,
    attr: &ast::Attr,
    cursor: &Ident,
) -> Option<ast::Path> {
    todo!()
}

/// Parses and resolves the path at the cursor position in the given attribute, if it is a derive.
/// This special case is required because the derive macro is a compiler builtin that discards the input derives.
pub fn try_resolve_derive_input(
    sema: &hir::Semantics<RootDatabase>,
    attr: &ast::Attr,
    cursor: &Ident,
) -> Option<PathResolution> {
    // let path = get_path_in_derive_attr(sema, attr, cursor)?;
    // let scope = sema.scope(attr.syntax());
    // // FIXME: This double resolve shouldn't be necessary
    // // It's only here so we prefer macros over other namespaces
    // match scope.speculative_resolve_as_mac(&path) {
    //     Some(mac) if mac.kind() == hir::MacroKind::Derive => Some(PathResolution::Macro(mac)),
    //     Some(_) => return None,
    //     None => scope
    //         .speculative_resolve(&path)
    //         .filter(|res| matches!(res, PathResolution::Def(ModuleDef::Module(_)))),
    // }
    todo!()
}

/// Picks the token with the highest rank returned by the passed in function.
pub fn pick_best_token(
    tokens: TokenAtOffset<SyntaxToken>,
    f: impl Fn(SyntaxKind) -> usize,
) -> Option<SyntaxToken> {
    todo!()
}

/// Converts the mod path struct into its ast representation.
pub fn mod_path_to_ast(path: &hir::ModPath) -> ast::Path {
    todo!()
    //     let _p = profile::span("mod_path_to_ast");

    //     let mut segments = Vec::new();
    //     let mut is_abs = false;
    //     match path.kind {
    //         hir::PathKind::Plain => {}
    //         hir::PathKind::Super(0) => segments.push(make::path_segment_self()),
    //         hir::PathKind::Super(n) => segments.extend((0..n).map(|_| make::path_segment_super())),
    //         hir::PathKind::DollarCrate(_) | hir::PathKind::Crate => {
    //             segments.push(make::path_segment_crate())
    //         }
    //         hir::PathKind::Abs => is_abs = true,
    //     }

    //     segments.extend(
    //         path.segments()
    //             .iter()
    //             .map(|segment| make::path_segment(make::name_ref(&segment.to_smol_str()))),
    //     );
    //     make::path_from_segments(segments, is_abs)
}

/// Iterates all `ModuleDef`s and `Impl` blocks of the given file.
pub fn visit_file_defs(
    sema: &Semantics<RootDatabase>,
    file_id: FileID,
    cb: &mut dyn FnMut(Definition),
) {
    todo!()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetCap {
    _private: (),
}

impl SnippetCap {
    pub const fn new(allow_snippets: bool) -> Option<SnippetCap> {
        if allow_snippets {
            Some(SnippetCap { _private: () })
        } else {
            None
        }
    }
}

/// Calls `cb` on each expression inside `expr` that is at "tail position".
/// Does not walk into `break` or `return` expressions.
/// Note that modifying the tree while iterating it will cause undefined iteration which might
/// potentially results in an out of bounds panic.
pub fn for_each_tail_expr(expr: &ast::Expr, cb: &mut dyn FnMut(&ast::Expr)) {
    todo!()
}

/// Calls `cb` on each break expr inside of `body` that is applicable for the given label.
pub fn for_each_break_expr(
    label: Option<ast::Label>,
    body: Option<ast::StmtList>,
    cb: &mut dyn FnMut(ast::BreakExpr),
) {
    todo!()
}

/// Checks if the given lint is equal or is contained by the other lint which may or may not be a group.
pub fn lint_eq_or_in_group(lint: &str, lint_is: &str) -> bool {
    if lint == lint_is {
        return true;
    }

    if let Some(group) = generated_lints::DEFAULT_LINT_GROUPS
        .iter()
        .chain(generated_lints::CLIPPY_LINT_GROUPS.iter())
        .chain(generated_lints::RUSTDOC_LINT_GROUPS.iter())
        .find(|&check| check.lint.label == lint_is)
    {
        group.children.contains(&lint)
    } else {
        false
    }
}
