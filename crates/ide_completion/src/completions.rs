//! This module defines an accumulator for completions which are going to be presented to user.

pub(crate) mod attribute;
pub(crate) mod dot;
pub(crate) mod flyimport;
pub(crate) mod fn_param;
pub(crate) mod keyword;
pub(crate) mod mod_;
pub(crate) mod pattern;
pub(crate) mod postfix;
pub(crate) mod qualified_path;
pub(crate) mod record;
pub(crate) mod snippet;
pub(crate) mod unqualified_path;

use std::iter;

use husky_lang_db::SymbolKind;

use crate::{
    item::Builder,
    render::{
        const_::render_const,
        enum_variant::render_variant,
        function::{render_fn, render_method},
        pattern::{render_struct_pat, render_variant_pat},
        render_field, render_resolution, render_tuple_field,
        struct_literal::render_struct_literal,
        type_alias::{render_type_alias, render_type_alias_with_eq},
        RenderContext,
    },
    CompletionContext, CompletionItem, CompletionItemKind,
};

/// Represents an in-progress set of completions being built.
#[derive(Debug, Default)]
pub struct Completions {
    buf: Vec<CompletionItem>,
}

impl From<Completions> for Vec<CompletionItem> {
    fn from(val: Completions) -> Self {
        val.buf
    }
}

impl Builder {
    /// Convenience method, which allows to add a freshly created completion into accumulator
    /// without binding it to the variable.
    pub(crate) fn add_to(self, acc: &mut Completions) {
        acc.add(self.build())
    }
}

impl Completions {
    fn add(&mut self, item: CompletionItem) {
        self.buf.push(item)
    }

    fn add_opt(&mut self, item: Option<CompletionItem>) {
        if let Some(item) = item {
            self.buf.push(item)
        }
    }

    pub(crate) fn add_all<I>(&mut self, items: I)
    where
        I: IntoIterator,
        I::Item: Into<CompletionItem>,
    {
        items.into_iter().for_each(|item| self.add(item.into()))
    }

    pub(crate) fn add_keyword(&mut self, ctx: &CompletionContext, keyword: &'static str) {
        todo!()
    }

    pub(crate) fn add_resolution(
        &mut self,
        ctx: &CompletionContext,
        local_name: hir::Name,
        resolution: &hir::ScopeDef,
    ) {
        todo!()
    }

    pub(crate) fn add_function(
        &mut self,
        ctx: &CompletionContext,
        func: hir::Function,
        local_name: Option<hir::Name>,
    ) {
        todo!()
    }

    pub(crate) fn add_method(
        &mut self,
        ctx: &CompletionContext,
        func: hir::Function,
        receiver: Option<hir::Name>,
        local_name: Option<hir::Name>,
    ) {
        todo!()
    }

    pub(crate) fn add_const(&mut self, ctx: &CompletionContext, konst: hir::Const) {
        todo!()
    }

    pub(crate) fn add_type_alias(&mut self, ctx: &CompletionContext, type_alias: hir::TypeAlias) {
        todo!()
    }

    pub(crate) fn add_type_alias_with_eq(
        &mut self,
        ctx: &CompletionContext,
        type_alias: hir::TypeAlias,
    ) {
        todo!()
    }

    pub(crate) fn add_qualified_enum_variant(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::Variant,
        path: hir::ModPath,
    ) {
        todo!()
    }

    pub(crate) fn add_enum_variant(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::Variant,
        local_name: Option<hir::Name>,
    ) {
        todo!()
    }

    pub(crate) fn add_field(
        &mut self,
        ctx: &CompletionContext,
        receiver: Option<hir::Name>,
        field: hir::Field,
        ty: &hir::Type,
    ) {
        todo!()
    }

    pub(crate) fn add_struct_literal(
        &mut self,
        ctx: &CompletionContext,
        strukt: hir::Struct,
        path: Option<hir::ModPath>,
        local_name: Option<hir::Name>,
    ) {
        todo!()
    }

    pub(crate) fn add_tuple_field(
        &mut self,
        ctx: &CompletionContext,
        receiver: Option<hir::Name>,
        field: usize,
        ty: &hir::Type,
    ) {
        todo!()
    }

    pub(crate) fn add_static_lifetime(&mut self, ctx: &CompletionContext) {
        todo!()
    }

    pub(crate) fn add_variant_pat(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::Variant,
        local_name: Option<hir::Name>,
    ) {
        todo!()
    }

    pub(crate) fn add_qualified_variant_pat(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::Variant,
        path: hir::ModPath,
    ) {
        todo!()
    }

    pub(crate) fn add_struct_pat(
        &mut self,
        ctx: &CompletionContext,
        strukt: hir::Struct,
        local_name: Option<hir::Name>,
    ) {
        todo!()
    }
}

/// Calls the callback for each variant of the provided enum with the path to the variant.
/// Skips variants that are visible with single segment paths.
fn enum_variants_with_paths(
    acc: &mut Completions,
    ctx: &CompletionContext,
    enum_: hir::Enum,
    cb: impl Fn(&mut Completions, &CompletionContext, hir::Variant, hir::ModPath),
) {
    todo!()
}
