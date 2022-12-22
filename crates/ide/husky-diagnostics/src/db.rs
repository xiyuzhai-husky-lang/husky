use husky_ast::AstDb;
use husky_entity_tree::EntityTreeDb;
use husky_term::Term;
use reserve::Reserve;
use salsa::DbWithJar;

use crate::*;

pub trait DiagnosticsDb: DbWithJar<DiagnosticsJar> + EntityTreeDb + AstDb {
    fn diagnostics_reserve(&self, module: Term) -> Arc<DiagnosticReserve>;
    fn print_diagnostics(&self) {
        todo!()
        // let modules = self.all_modules();
        // for module in modules.iter() {
        //     let diagnostic_reserve = self.diagnostics_reserve(*module);
        //     p!(module);
        //     p!(self.module_file(*module));
        //     p!(diagnostic_reserve.data());
        // }
    }

    fn all_diagnostics(&self) -> Vec<(Term, Diagnostic)> {
        todo!()
        // let mut diagnostics = vec![];
        // for module in self.all_modules() {
        //     let diagnostics_reserve = self.diagnostics_reserve(module);
        //     diagnostics.extend(
        //         diagnostics_reserve
        //             .data()
        //             .iter()
        //             .map(|d| (module, d.clone())),
        //     );
        // }
        // diagnostics
    }
}

impl<T> DiagnosticsDb for T
where
    T: DbWithJar<DiagnosticsJar> + EntityTreeDb + AstDb,
{
    fn diagnostics_reserve(&self, _module: Term) -> Arc<DiagnosticReserve> {
        todo!()
    }
}

fn diagnostics_reserve(this: &dyn DiagnosticsDb, module: Term) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_module_diagnostics(
        this, module,
    )))
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
