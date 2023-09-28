use husky_sema_expr::SemaExprDb;

use crate::*;
use husky_print_utils::p;
use reserve::Reserve;
use salsa::DbWithJar;
use salsa::DebugWithDb;

pub trait DiagnosticsDb: DbWithJar<DiagnosticsJar> + SemaExprDb {
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet;
}

impl<Db> DiagnosticsDb for Db
where
    Db: DbWithJar<DiagnosticsJar> + SemaExprDb,
{
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet {
        diagnostic_sheet(self, module_path)
    }
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
