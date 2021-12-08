//! Computes color for a single element.
use common::*;

use hir::{AsAssocItem, HasVisibility, Semantics};
use husky_lang_db::{
    defs::{Definition, NameClass, NameRefClass},
    helpers::{try_resolve_derive_input, FamousDefs},
    HuskyLangDatabase, SymbolKind,
};
use rustc_hash::FxHashMap;
use syntax::{
    ast, NodeOrToken, SyntaxElement,
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken,
};

use crate::{
    syntax_highlighting::tags::{HlOperator, HlPunct},
    Highlight, HlMod, HlTag,
};

pub(super) fn element(
    sema: &Semantics<HuskyLangDatabase>,
    krate: Option<hir::Crate>,
    bindings_shadow_count: &mut FxHashMap<hir::Name, u32>,
    syntactic_name_ref_highlighting: bool,
    element: SyntaxElement,
) -> Option<(Highlight, Option<u64>)> {
    todo!()
}

fn token(
    sema: &Semantics<HuskyLangDatabase>,
    krate: Option<hir::Crate>,
    token: SyntaxToken,
) -> Option<Highlight> {
    todo!()
}

fn node(
    sema: &Semantics<HuskyLangDatabase>,
    krate: Option<hir::Crate>,
    bindings_shadow_count: &mut FxHashMap<hir::Name, u32>,
    syntactic_name_ref_highlighting: bool,
    node: SyntaxNode,
) -> Option<(Highlight, Option<u64>)> {
    todo!()
}

fn highlight_name_ref_in_attr(
    sema: &Semantics<HuskyLangDatabase>,
    name_ref: ast::NameRef,
) -> Highlight {
    todo!()
}

fn highlight_name_ref(
    sema: &Semantics<HuskyLangDatabase>,
    krate: Option<hir::Crate>,
    bindings_shadow_count: &mut FxHashMap<hir::Name, u32>,
    binding_hash: &mut Option<u64>,
    syntactic_name_ref_highlighting: bool,
    name_ref: ast::NameRef,
) -> Highlight {
    todo!()
}

fn highlight_name(
    sema: &Semantics<HuskyLangDatabase>,
    bindings_shadow_count: &mut FxHashMap<hir::Name, u32>,
    binding_hash: &mut Option<u64>,
    krate: Option<hir::Crate>,
    name: ast::Name,
) -> Highlight {
    todo!()
}

fn calc_binding_hash(name: &hir::Name, shadow_count: u32) -> u64 {
    fn hash<T: std::hash::Hash + std::fmt::Debug>(x: T) -> u64 {
        use std::{collections::hash_map::DefaultHasher, hash::Hasher};

        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        hasher.finish()
    }

    hash((name, shadow_count))
}

fn highlight_def(
    sema: &Semantics<HuskyLangDatabase>,
    krate: Option<hir::Crate>,
    def: Definition,
) -> Highlight {
    todo!()
}

fn highlight_method_call_by_name_ref(
    sema: &Semantics<HuskyLangDatabase>,
    krate: Option<hir::Crate>,
    name_ref: &ast::NameRef,
) -> Option<Highlight> {
    todo!()
}

fn highlight_method_call(
    sema: &Semantics<HuskyLangDatabase>,
    krate: Option<hir::Crate>,
    method_call: &ast::MethodCallExpr,
) -> Option<Highlight> {
    todo!()
}

fn highlight_name_by_syntax(name: ast::Name) -> Highlight {
    todo!()
}

fn highlight_name_ref_by_syntax(
    name: ast::NameRef,
    sema: &Semantics<HuskyLangDatabase>,
    krate: Option<hir::Crate>,
) -> Highlight {
    todo!()
}

fn is_consumed_lvalue(node: &SyntaxNode, local: &hir::Local, db: &HuskyLangDatabase) -> bool {
    todo!()
}

/// Returns true if the parent nodes of `node` all match the `SyntaxKind`s in `kinds` exactly.
fn parents_match(mut node: NodeOrToken<SyntaxNode, SyntaxToken>, mut kinds: &[SyntaxKind]) -> bool {
    todo!()
}

fn is_child_of_impl(token: &SyntaxToken) -> bool {
    todo!()
}
