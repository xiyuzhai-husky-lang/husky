//! Completion of names from the current scope, e.g. locals and imported items.

use hir::ScopeDef;
use syntax::ast;

use crate::{CompletionContext, Completions};

pub(crate) fn complete_unqualified_path(acc: &mut Completions, ctx: &CompletionContext) {
    todo!()
}
