//! Complete fields in record literals and patterns.
use ide_db::{helpers::FamousDefs, SymbolKind};
use syntax::ast::Expr;

use crate::{CompletionContext, CompletionItem, CompletionItemKind, Completions};

pub(crate) fn complete_record(acc: &mut Completions, ctx: &CompletionContext) -> Option<()> {
    todo!()
}

pub(crate) fn complete_record_literal(
    acc: &mut Completions,
    ctx: &CompletionContext,
) -> Option<()> {
    todo!()
}
