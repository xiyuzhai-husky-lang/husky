use either::Either;
use hir::{db::AstDatabase, InFile};
use ide_db::{assists::Assist, source_change::SourceChange};
use rustc_hash::FxHashMap;
use stdx::format_to;
use text_edit::TextEdit;

use crate::{fix, Diagnostic, DiagnosticsContext};

// Diagnostic: missing-fields
//
// This diagnostic is triggered if record lacks some fields that exist in the corresponding structure.
//
// Example:
//
// ```rust
// struct A { a: u8, b: u8 }
//
// let a = A { a: 10 };
// ```
pub(crate) fn missing_fields(ctx: &DiagnosticsContext<'_>, d: &hir::MissingFields) -> Diagnostic {
    todo!()
}

fn fixes(ctx: &DiagnosticsContext<'_>, d: &hir::MissingFields) -> Option<Vec<Assist>> {
    todo!()
}
