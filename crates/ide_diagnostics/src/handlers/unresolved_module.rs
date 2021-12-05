use hir::db::AstDatabase;
use ide_db::{assists::Assist, base_db::AnchoredPathBuf, source_change::FileSystemEdit};

use crate::{fix, Diagnostic, DiagnosticsContext};

// Diagnostic: unresolved-module
//
// This diagnostic is triggered if rust-analyzer is unable to discover referred module.
pub(crate) fn unresolved_module(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::UnresolvedModule,
) -> Diagnostic {
    todo!()
}

fn fixes(ctx: &DiagnosticsContext<'_>, d: &hir::UnresolvedModule) -> Option<Vec<Assist>> {
    todo!()
}
