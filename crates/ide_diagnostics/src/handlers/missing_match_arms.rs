use hir::InFile;

use crate::{Diagnostic, DiagnosticsContext};

// Diagnostic: missing-match-arm
//
// This diagnostic is triggered if `match` block is missing one or more match arms.
pub(crate) fn missing_match_arms(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::MissingMatchArms,
) -> Diagnostic {
    todo!()
}
