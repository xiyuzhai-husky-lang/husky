use hir::db::AstDatabase;
use ide_db::source_change::SourceChange;
use text_edit::TextEdit;

use crate::{fix, Assist, Diagnostic, DiagnosticsContext};

// Diagnostic: add-reference-here
//
// This diagnostic is triggered when there's a missing referencing of expression.
pub(crate) fn add_reference_here(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::AddReferenceHere,
) -> Diagnostic {
    todo!()
}

fn fixes(ctx: &DiagnosticsContext<'_>, d: &hir::AddReferenceHere) -> Option<Vec<Assist>> {
    todo!()
}
