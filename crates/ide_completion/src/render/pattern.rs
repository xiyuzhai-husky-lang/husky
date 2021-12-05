//! Renderer for patterns.

use hir::{db::HirDatabase, HasVisibility, Name, StructKind};
use ide_db::helpers::SnippetCap;
use itertools::Itertools;
use syntax::SmolStr;

use crate::{
    context::{ParamKind, PatternContext},
    render::RenderContext,
    CompletionItem, CompletionItemKind,
};

pub(crate) fn render_struct_pat(
    ctx: RenderContext<'_>,
    strukt: hir::Struct,
    local_name: Option<Name>,
) -> Option<CompletionItem> {
    todo!()
}

pub(crate) fn render_variant_pat(
    ctx: RenderContext<'_>,
    variant: hir::Variant,
    local_name: Option<Name>,
    path: Option<hir::ModPath>,
) -> Option<CompletionItem> {
    todo!()
}

fn build_completion(ctx: RenderContext<'_>, name: SmolStr, pat: String) -> CompletionItem {
    todo!()
}

fn render_pat(
    ctx: &RenderContext<'_>,
    name: &str,
    kind: StructKind,
    fields: &[hir::Field],
    fields_omitted: bool,
) -> Option<String> {
    todo!()
}

fn render_record_as_pat(
    db: &dyn HirDatabase,
    snippet_cap: Option<SnippetCap>,
    fields: &[hir::Field],
    name: &str,
    fields_omitted: bool,
) -> String {
    todo!()
}

fn render_tuple_as_pat(fields: &[hir::Field], name: &str, fields_omitted: bool) -> String {
    todo!()
}

fn visible_fields(
    ctx: &RenderContext<'_>,
    fields: &[hir::Field],
) -> Option<(Vec<hir::Field>, bool)> {
    todo!()
}
