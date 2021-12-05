use hir::{db::AstDatabase, InFile};
use ide_db::source_change::SourceChange;
use syntax::{ast, TextRange};
use text_edit::TextEdit;

use crate::{fix, Assist, Diagnostic, DiagnosticsContext, Severity};

// Diagnostic: replace-filter-map-next-with-find-map
//
// This diagnostic is triggered when `.filter_map(..).next()` is used, rather than the more concise `.find_map(..)`.
pub(crate) fn replace_filter_map_next_with_find_map(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::ReplaceFilterMapNextWithFindMap,
) -> Diagnostic {
    todo!()
}

fn fixes(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::ReplaceFilterMapNextWithFindMap,
) -> Option<Vec<Assist>> {
    todo!()
}
