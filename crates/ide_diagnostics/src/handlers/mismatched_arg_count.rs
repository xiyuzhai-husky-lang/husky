use ide_db::base_db::{FileRange, SourceDatabase};
use syntax::ast;

use crate::{Diagnostic, DiagnosticsContext};

// Diagnostic: mismatched-arg-count
//
// This diagnostic is triggered if a function is invoked with an incorrect amount of arguments.
pub(crate) fn mismatched_arg_count(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::MismatchedArgCount,
) -> Diagnostic {
    todo!()
}

fn invalid_args_range(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::MismatchedArgCount,
) -> Result<FileRange, FileRange> {
    todo!()
}
