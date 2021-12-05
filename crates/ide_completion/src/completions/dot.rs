//! Completes references after dot (fields and method calls).

use either::Either;
use hir::ScopeDef;
use rustc_hash::FxHashSet;

use crate::{context::CompletionContext, Completions};

/// Complete dot accesses, i.e. fields or methods.
pub(crate) fn complete_dot(acc: &mut Completions, ctx: &CompletionContext) {
    todo!()
}

fn complete_undotted_self(acc: &mut Completions, ctx: &CompletionContext) {
    todo!()
}

fn complete_fields(
    ctx: &CompletionContext,
    receiver: &hir::Type,
    mut f: impl FnMut(Either<hir::Field, usize>, hir::Type),
) {
    todo!()
}

fn complete_methods(
    ctx: &CompletionContext,
    receiver: &hir::Type,
    mut f: impl FnMut(hir::Function),
) {
    todo!()
}
