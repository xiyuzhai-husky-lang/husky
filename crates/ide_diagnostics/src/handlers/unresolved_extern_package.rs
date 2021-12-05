use crate::{Diagnostic, DiagnosticsContext};

// Diagnostic: unresolved-extern-crate
//
// This diagnostic is triggered if rust-analyzer is unable to discover referred extern crate.
pub(crate) fn unresolved_extern_package(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::UnresolvedExternPackage,
) -> Diagnostic {
    todo!()
}
