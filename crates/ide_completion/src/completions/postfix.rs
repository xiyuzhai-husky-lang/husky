//! Postfix completions, like `Ok(10).ifl$0` => `if let Ok() = Ok(10) { $0 }`.

mod format_like;

use common::*;

use hir::Documentation;
use husky_lang_db::{
    helpers::{insert_use::ImportScope, FamousDefs, SnippetCap},
    ty_filter::TryEnum,
};
use syntax::ast;
use text_edit::TextEdit;

use crate::{
    completions::postfix::format_like::add_format_like_completions, context::CompletionContext,
    item::Builder, CompletionItem, CompletionItemKind, CompletionRelevance, Completions,
    SnippetScope,
};

pub(crate) fn complete_postfix(acc: &mut Completions, ctx: &CompletionContext) {
    todo!()
}

fn get_receiver_text(receiver: &ast::Expr, receiver_is_ambiguous_float_literal: bool) -> String {
    todo!()
}

fn include_references(initial_element: &ast::Expr) -> ast::Expr {
    todo!()
}

fn build_postfix_snippet_builder<'ctx>(
    ctx: &'ctx CompletionContext,
    cap: SnippetCap,
    receiver: &'ctx ast::Expr,
) -> Option<Box<dyn Fn(&str, &str, &str) -> Builder + 'ctx>> {
    todo!()
}

fn add_custom_postfix_completions(
    acc: &mut Completions,
    ctx: &CompletionContext,
    postfix_snippet: impl Fn(&str, &str, &str) -> Builder,
    receiver_text: &str,
) -> Option<()> {
    todo!()
}
