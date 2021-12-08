//! Renderer for `enum` variants.

use std::iter;

use hir::HirDisplay;
use husky_lang_db::SymbolKind;
use itertools::Itertools;

use crate::{
    item::{CompletionItem, ImportEdit},
    render::{builder_ext::Params, compute_ref_match, compute_type_match, RenderContext},
    CompletionRelevance,
};

pub(crate) fn render_variant(
    ctx: RenderContext<'_>,
    import_to_add: Option<ImportEdit>,
    local_name: Option<hir::Name>,
    variant: hir::Variant,
    path: Option<hir::ModPath>,
) -> CompletionItem {
    let _p = profile::span("render_enum_variant");
    EnumRender::new(ctx, local_name, variant, path).render(import_to_add)
}

#[derive(Debug)]
struct EnumRender<'a> {
    ctx: RenderContext<'a>,
    variant: hir::Variant,
    path: Option<hir::ModPath>,
    qualified_name: hir::ModPath,
    short_qualified_name: hir::ModPath,
    variant_kind: hir::StructKind,
}

impl<'a> EnumRender<'a> {
    fn new(
        ctx: RenderContext<'a>,
        local_name: Option<hir::Name>,
        variant: hir::Variant,
        path: Option<hir::ModPath>,
    ) -> EnumRender<'a> {
        todo!()
    }
    fn render(self, import_to_add: Option<ImportEdit>) -> CompletionItem {
        todo!()
    }

    fn detail(&self) -> String {
        todo!()
    }
}
