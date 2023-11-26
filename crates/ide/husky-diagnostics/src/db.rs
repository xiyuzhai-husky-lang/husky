use crate::*;
use husky_sema_expr::SemaExprDb;

pub trait DiagnosticsDb {
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet;
}

impl DiagnosticsDb for ::salsa::Db {
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet {
        diagnostic_sheet(self, module_path)
    }
}
