use crate::{Diagnostic, DiagnosticsContext};

// Diagnostic: unresolved-import
//
// This diagnostic is triggered if rust-analyzer is unable to resolve a path in
// a `use` declaration.
pub(crate) fn unresolved_import(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::UnresolvedImport,
) -> Diagnostic {
    todo!()
}
