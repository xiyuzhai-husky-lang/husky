use hir::{db::AstDatabase, HasSource, HirDisplay, Semantics};
use ide_db::{base_db::FileID, source_change::SourceChange, RootDatabase};
use syntax::ast;
use text_edit::TextEdit;

use crate::{fix, Assist, Diagnostic, DiagnosticsContext};

// Diagnostic: no-such-field
//
// This diagnostic is triggered if created structure does not have field provided in record.
pub(crate) fn no_such_field(ctx: &DiagnosticsContext<'_>, d: &hir::NoSuchField) -> Diagnostic {
    todo!()
}

fn fixes(ctx: &DiagnosticsContext<'_>, d: &hir::NoSuchField) -> Option<Vec<Assist>> {
    todo!()
}

fn missing_record_expr_field_fixes(
    sema: &Semantics<RootDatabase>,
    usage_file_id: FileID,
    record_expr_field: &ast::RecordExprField,
) -> Option<Vec<Assist>> {
    todo!()
}
