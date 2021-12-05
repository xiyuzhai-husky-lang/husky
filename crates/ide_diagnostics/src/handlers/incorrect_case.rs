use hir::{db::AstDatabase, InFile};
use ide_db::{assists::Assist, defs::NameClass};

use crate::{
    // references::rename::rename_with_semantics,
    unresolved_fix,
    Diagnostic,
    DiagnosticsContext,
    Severity,
};

// Diagnostic: incorrect-ident-case
//
// This diagnostic is triggered if an item name doesn't follow https://doc.rust-lang.org/1.0.0/style/style/naming/README.html[Rust naming convention].
pub(crate) fn incorrect_case(ctx: &DiagnosticsContext<'_>, d: &hir::IncorrectCase) -> Diagnostic {
    todo!()
}

fn fixes(ctx: &DiagnosticsContext<'_>, d: &hir::IncorrectCase) -> Option<Vec<Assist>> {
    todo!()
}
