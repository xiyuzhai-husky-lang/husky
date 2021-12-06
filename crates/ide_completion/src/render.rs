//! `render` module provides utilities for rendering completion suggestions
//! into code pieces that will be presented to user.

pub(crate) mod const_;
pub(crate) mod enum_variant;
pub(crate) mod function;
pub(crate) mod pattern;
pub(crate) mod struct_literal;
pub(crate) mod type_alias;

mod builder_ext;

use common::*;

use hir::{AsAssocItem, HirDisplay};
use ide_db::{
    helpers::{item_name, SnippetCap},
    IdeDatabase, SymbolKind,
};
use syntax::{SmolStr, SyntaxKind};

use crate::{
    context::{PathCompletionContext, PathKind},
    item::{CompletionRelevanceTypeMatch, ImportEdit},
    render::{enum_variant::render_variant, function::render_fn},
    CompletionItem, CompletionItemKind, CompletionRelevance,
};
/// Interface for data and methods required for items rendering.
#[derive(Debug)]
pub(crate) struct RenderContext<'a> {
    completion: &'a CompletionContext<'a>,
}

impl<'a> RenderContext<'a> {
    pub(crate) fn new(completion: &'a CompletionContext<'a>) -> RenderContext<'a> {
        RenderContext { completion }
    }

    fn snippet_cap(&self) -> Option<SnippetCap> {
        todo!()
    }

    fn db(&self) -> &'a IdeDatabase {
        todo!()
    }

    fn source_range(&self) -> TextRange {
        todo!()
    }

    fn is_deprecated_assoc_item(&self, as_assoc_item: impl AsAssocItem) -> bool {
        todo!()
    }
}

pub(crate) fn render_field(
    ctx: RenderContext<'_>,
    receiver: Option<hir::Name>,
    field: hir::Field,
    ty: &hir::Type,
) -> CompletionItem {
    todo!()
}

pub(crate) fn render_tuple_field(
    ctx: RenderContext<'_>,
    receiver: Option<hir::Name>,
    field: usize,
    ty: &hir::Type,
) -> CompletionItem {
    todo!()
}

pub(crate) fn render_resolution(
    ctx: RenderContext<'_>,
    local_name: hir::Name,
    resolution: &hir::ScopeDef,
) -> Option<CompletionItem> {
    render_resolution_(ctx, local_name, None, resolution)
}

pub(crate) fn render_resolution_with_import(
    ctx: RenderContext<'_>,
    import_edit: ImportEdit,
) -> Option<CompletionItem> {
    todo!()
}

fn render_resolution_(
    ctx: RenderContext<'_>,
    local_name: hir::Name,
    import_to_add: Option<ImportEdit>,
    resolution: &hir::ScopeDef,
) -> Option<CompletionItem> {
    todo!()
}

fn scope_def_docs(db: &IdeDatabase, resolution: &hir::ScopeDef) -> Option<hir::Documentation> {
    todo!()
}

fn scope_def_is_deprecated(ctx: &RenderContext<'_>, resolution: &hir::ScopeDef) -> bool {
    todo!()
}
#[derive(Debug)]
pub struct CompletionContext<'a> {
    phantom: std::marker::PhantomData<&'a ()>,
}
fn compute_type_match(
    ctx: &CompletionContext,
    completion_ty: &hir::Type,
) -> Option<CompletionRelevanceTypeMatch> {
    todo!()
}

fn compute_exact_name_match(ctx: &CompletionContext, completion_name: &str) -> bool {
    todo!()
}

fn compute_ref_match(
    ctx: &CompletionContext,
    completion_ty: &hir::Type,
) -> Option<hir::Mutability> {
    todo!()
}
