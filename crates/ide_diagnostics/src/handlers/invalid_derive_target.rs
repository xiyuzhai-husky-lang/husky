use crate::{Diagnostic, DiagnosticsContext, Severity};

// Diagnostic: invalid-derive-target
//
// This diagnostic is shown when the derive attribute is used on an item other than a `struct`,
// `enum` or `union`.
pub(crate) fn invalid_derive_target(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::InvalidDeriveTarget,
) -> Diagnostic {
    todo!()
}
