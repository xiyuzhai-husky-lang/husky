//! Completion of paths, i.e. `some::prefix::$0`.

use std::iter;

use rustc_hash::FxHashSet;

use crate::{context::PathCompletionContext, CompletionContext, Completions};

pub(crate) fn complete_qualified_path(acc: &mut Completions, ctx: &CompletionContext) {
    todo!()
}

fn add_assoc_item(acc: &mut Completions, ctx: &CompletionContext, item: hir::AssocItem) {
    todo!()
}

fn add_enum_variants(acc: &mut Completions, ctx: &CompletionContext, e: hir::Enum) {
    todo!()
}
