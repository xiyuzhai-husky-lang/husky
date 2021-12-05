//! Suggests shortening `Foo { field: field }` to `Foo { field }` in both
//! expressions and patterns.

use ide_db::{base_db::FileID, source_change::SourceChange};
use syntax::{ast, SyntaxNode};
use text_edit::TextEdit;

use crate::{fix, Diagnostic, Severity};

pub(crate) fn field_shorthand(acc: &mut Vec<Diagnostic>, file_id: FileID, node: &SyntaxNode) {
    todo!()
}

fn check_expr_field_shorthand(
    acc: &mut Vec<Diagnostic>,
    file_id: FileID,
    record_expr: ast::RecordExpr,
) {
    todo!()
}

fn check_pat_field_shorthand(
    acc: &mut Vec<Diagnostic>,
    file_id: FileID,
    record_pat: ast::RecordPat,
) {
    todo!()
}
