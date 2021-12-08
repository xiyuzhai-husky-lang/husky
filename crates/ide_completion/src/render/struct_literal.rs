//! Renderer for `struct` literal.

use hir::{db::HirDatabase, Name, StructKind};
use husky_lang_db::helpers::SnippetCap;
use itertools::Itertools;
use syntax::SmolStr;

use crate::{render::RenderContext, CompletionItem, CompletionItemKind};

pub(crate) fn render_struct_literal(
    ctx: RenderContext<'_>,
    strukt: hir::Struct,
    path: Option<hir::ModPath>,
    local_name: Option<Name>,
) -> Option<CompletionItem> {
    todo!()
}

fn build_completion(ctx: RenderContext<'_>, name: SmolStr, literal: String) -> CompletionItem {
    todo!()
}

fn render_literal(
    ctx: &RenderContext<'_>,
    path: Option<hir::ModPath>,
    name: &str,
    kind: StructKind,
    fields: &[hir::Field],
) -> Option<String> {
    todo!()
}

fn render_record_as_literal(
    db: &dyn HirDatabase,
    snippet_cap: Option<SnippetCap>,
    fields: &[hir::Field],
    name: &str,
) -> String {
    todo!()
}

fn render_tuple_as_literal(fields: &[hir::Field], name: &str) -> String {
    format!(
        "{name}({})",
        fields
            .iter()
            .enumerate()
            .map(|(idx, _)| format!("${}", idx + 1))
            .format(", "),
        name = name
    )
}

fn visible_fields(
    ctx: &RenderContext<'_>,
    fields: &[hir::Field],
) -> Option<(Vec<hir::Field>, bool)> {
    todo!()
}
