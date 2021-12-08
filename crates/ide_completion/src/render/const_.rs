//! Renderer for `const` fields.

use hir::AsAssocItem;
use husky_lang_db::SymbolKind;
use syntax::ast::Const;

use crate::{item::CompletionItem, render::RenderContext};

pub(crate) fn render_const(ctx: RenderContext<'_>, const_: hir::Const) -> Option<CompletionItem> {
    let _p = profile::span("render_const");
    ConstRender::new(ctx, const_)?.render()
}

#[derive(Debug)]
struct ConstRender<'a> {
    ctx: RenderContext<'a>,
    const_: hir::Const,
    ast_node: Const,
}

impl<'a> ConstRender<'a> {
    fn new(ctx: RenderContext<'a>, const_: hir::Const) -> Option<ConstRender<'a>> {
        todo!()
    }

    fn render(self) -> Option<CompletionItem> {
        todo!()
    }

    fn detail(&self) -> String {
        todo!()
    }
}
