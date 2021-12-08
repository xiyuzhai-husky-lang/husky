//! Renderer for type aliases.

use hir::{AsAssocItem, HasSource};
use husky_lang_db::SymbolKind;
use syntax::ast::TypeAlias;

use crate::{item::CompletionItem, render::RenderContext};

pub(crate) fn render_type_alias(
    ctx: RenderContext<'_>,
    type_alias: hir::TypeAlias,
) -> Option<CompletionItem> {
    let _p = profile::span("render_type_alias");
    TypeAliasRender::new(ctx, type_alias)?.render(false)
}

pub(crate) fn render_type_alias_with_eq(
    ctx: RenderContext<'_>,
    type_alias: hir::TypeAlias,
) -> Option<CompletionItem> {
    let _p = profile::span("render_type_alias_with_eq");
    TypeAliasRender::new(ctx, type_alias)?.render(true)
}

#[derive(Debug)]
struct TypeAliasRender<'a> {
    ctx: RenderContext<'a>,
    type_alias: hir::TypeAlias,
    ast_node: TypeAlias,
}

impl<'a> TypeAliasRender<'a> {
    fn new(ctx: RenderContext<'a>, type_alias: hir::TypeAlias) -> Option<TypeAliasRender<'a>> {
        todo!()
    }

    fn render(self, with_eq: bool) -> Option<CompletionItem> {
        todo!()
    }

    fn detail(&self) -> String {
        todo!()
    }
}
