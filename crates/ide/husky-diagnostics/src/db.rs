use husky_expr_ty::ExprTypeDb;

use reserve::Reserve;
use salsa::DbWithJar;

use crate::*;

pub trait DiagnosticsDb: DbWithJar<DiagnosticsJar> + ExprTypeDb {
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet;
}

impl<T> DiagnosticsDb for T
where
    T: DbWithJar<DiagnosticsJar> + ExprTypeDb,
{
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet {
        diagnostic_sheet(self, module_path)
    }
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
