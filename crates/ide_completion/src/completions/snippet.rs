//! This file provides snippet completions, like `pd` => `eprintln!(...)`.

use hir::Documentation;
use ide_db::helpers::{insert_use::ImportScope, SnippetCap};

use crate::{
    context::PathCompletionContext, item::Builder, CompletionContext, CompletionItem,
    CompletionItemKind, Completions, SnippetScope,
};

fn snippet(ctx: &CompletionContext, cap: SnippetCap, label: &str, snippet: &str) -> Builder {
    todo!()
}

pub(crate) fn complete_expr_snippet(acc: &mut Completions, ctx: &CompletionContext) {
    todo!()
}

pub(crate) fn complete_item_snippet(acc: &mut Completions, ctx: &CompletionContext) {
    todo!()
}

fn add_custom_completions(
    acc: &mut Completions,
    ctx: &CompletionContext,
    cap: SnippetCap,
    scope: SnippetScope,
) -> Option<()> {
    todo!()
}
