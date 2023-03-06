use husky_ast::AstDb;
use husky_defn::DefnDb;
use husky_entity_tree::EntityTreeDb;
use husky_expr_ty::ExprTermDb;
use husky_term::Term;
use reserve::Reserve;
use salsa::DbWithJar;

use crate::*;

pub trait DiagnosticsDb: DbWithJar<DiagnosticsJar> + ExprTermDb {
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet;
}

impl<T> DiagnosticsDb for T
where
    T: DbWithJar<DiagnosticsJar> + ExprTermDb,
{
    fn diagnostic_sheet(&self, module_path: ModulePath) -> DiagnosticSheet {
        diagnostic_sheet(self, module_path)
    }
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
