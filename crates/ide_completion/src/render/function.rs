//! Renderer for function calls.

use either::Either;
use hir::{AsAssocItem, HasSource, HirDisplay};
use ide_db::SymbolKind;
use itertools::Itertools;
use syntax::ast;

use crate::{
    item::{CompletionItem, CompletionItemKind, CompletionRelevance, ImportEdit},
    render::{
        builder_ext::Params, compute_exact_name_match, compute_ref_match, compute_type_match,
        RenderContext,
    },
};

pub(crate) fn render_fn(
    ctx: RenderContext<'_>,
    import_to_add: Option<ImportEdit>,
    local_name: Option<hir::Name>,
    fn_: hir::Function,
) -> Option<CompletionItem> {
    let _p = profile::span("render_fn");
    Some(FunctionRender::new(ctx, None, local_name, fn_, false)?.render(import_to_add))
}

pub(crate) fn render_method(
    ctx: RenderContext<'_>,
    import_to_add: Option<ImportEdit>,
    receiver: Option<hir::Name>,
    local_name: Option<hir::Name>,
    fn_: hir::Function,
) -> Option<CompletionItem> {
    let _p = profile::span("render_method");
    Some(FunctionRender::new(ctx, receiver, local_name, fn_, true)?.render(import_to_add))
}

#[derive(Debug)]
struct FunctionRender<'a> {
    ctx: RenderContext<'a>,
    name: hir::Name,
    receiver: Option<hir::Name>,
    func: hir::Function,
    /// NB: having `ast::Fn` here might or might not be a good idea. The problem
    /// with it is that, to get an `ast::`, you want to parse the corresponding
    /// source file. So, when flyimport completions suggest a bunch of
    /// functions, we spend quite some time parsing many files.
    ///
    /// We need ast because we want to access parameter names (patterns). We can
    /// add them to the hir of the function itself, but parameter names are not
    /// something hir cares otherwise.
    ///
    /// Alternatively we can reconstruct params from the function body, but that
    /// would require parsing anyway.
    ///
    /// It seems that just using `ast` is the best choice -- most of parses
    /// should be cached anyway.
    ast_node: ast::Fn,
    is_method: bool,
}

impl<'a> FunctionRender<'a> {
    fn new(
        ctx: RenderContext<'a>,
        receiver: Option<hir::Name>,
        local_name: Option<hir::Name>,
        fn_: hir::Function,
        is_method: bool,
    ) -> Option<FunctionRender<'a>> {
        todo!()
    }

    fn render(self, import_to_add: Option<ImportEdit>) -> CompletionItem {
        todo!()
    }

    fn detail(&self) -> String {
        todo!()
    }

    fn params_display(&self) -> String {
        todo!()
    }

    fn ty_display(&self) -> String {
        todo!()
    }

    fn params(&self) -> Params {
        todo!()
    }

    fn kind(&self) -> CompletionItemKind {
        todo!()
    }
}
