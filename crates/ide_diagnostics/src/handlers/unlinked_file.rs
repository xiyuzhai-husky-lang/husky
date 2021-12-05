//! Diagnostic emitted for files that aren't part of any crate.

use hir::db::DefDatabase;
use ide_db::{
    base_db::{FileID, FileLoader, SourceDatabase, SourceDatabaseExt},
    source_change::SourceChange,
    RootDatabase,
};
use syntax::{ast, TextRange, TextSize};
use text_edit::TextEdit;

use crate::{fix, Assist, Diagnostic, DiagnosticsContext, Severity};

// Diagnostic: unlinked-file
//
// This diagnostic is shown for files that are not included in any crate, or files that are part of
// crates rust-analyzer failed to discover. The file will not have IDE features available.
pub(crate) fn unlinked_file(ctx: &DiagnosticsContext, acc: &mut Vec<Diagnostic>, file_id: FileID) {
    todo!()
}

fn fixes(ctx: &DiagnosticsContext, file_id: FileID) -> Option<Vec<Assist>> {
    todo!()
}

fn make_fixes(
    db: &RootDatabase,
    parent_file_id: FileID,
    new_mod_name: &str,
    added_file_id: FileID,
) -> Option<Vec<Assist>> {
    todo!()
}
