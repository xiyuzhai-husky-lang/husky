use hir::db::AstDatabase;
use ide_db::{assists::Assist, helpers::for_each_tail_expr, source_change::SourceChange};
use text_edit::TextEdit;

use crate::{fix, Diagnostic, DiagnosticsContext};

// Diagnostic: missing-ok-or-some-in-tail-expr
//
// This diagnostic is triggered if a block that should return `Result` returns a value not wrapped in `Ok`,
// or if a block that should return `Option` returns a value not wrapped in `Some`.
//
// Example:
//
// ```rust
// fn foo() -> Result<u8, ()> {
//     10
// }
// ```
pub(crate) fn missing_ok_or_some_in_tail_expr(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::MissingOkOrSomeInTailExpr,
) -> Diagnostic {
    todo!()
}

fn fixes(ctx: &DiagnosticsContext<'_>, d: &hir::MissingOkOrSomeInTailExpr) -> Option<Vec<Assist>> {
    todo!()
}
