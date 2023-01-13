use husky_ast::AstDb;
use husky_defn::DefnDb;
use husky_entity_tree::EntityTreeDb;
use husky_term::Term;
use reserve::Reserve;
use salsa::DbWithJar;

use crate::*;

pub trait DiagnosticsDb: DbWithJar<DiagnosticsJar> + DefnDb {
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet;
}

impl<T> DiagnosticsDb for T
where
    T: DbWithJar<DiagnosticsJar> + DefnDb,
{
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet {
        diagnostic_sheet(self, module_path)
    }
}

fn diagnostics_reserve(this: &dyn DiagnosticsDb, module: Term) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_module_diagnostics(
        this, module,
    )))
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
