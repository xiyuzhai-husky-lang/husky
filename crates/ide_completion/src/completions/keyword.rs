//! Completes keywords, except:
//! - `self`, `super` and `crate`, as these are considered part of path completions.
//! - `await`, as this is a postfix completion we handle this in the postfix completions.

use syntax::SyntaxKind;

use crate::{
    context::PathCompletionContext, CompletionContext, CompletionItem, CompletionItemKind,
    Completions,
};

pub(crate) fn complete_expr_keyword(acc: &mut Completions, ctx: &CompletionContext) {
    todo!()
}

fn add_keyword(acc: &mut Completions, ctx: &CompletionContext, kw: &str, snippet: &str) {
    todo!()
}
