use husky_ast::AstQueryGroup;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::EntityDefnQueryGroup;
use husky_entity_syntax::EntitySyntaxQueryGroup;
use infer_total::InferQueryGroup;
use reserve::Reserve;

use crate::*;

#[salsa::query_group(DiagnosticSalsaQueryGroupStorage)]
pub trait DiagnosticSalsaQuery:
    EntitySyntaxQueryGroup + AstQueryGroup + InferQueryGroup + EntityDefnQueryGroup
{
    fn diagnostics_reserve(&self, module: EntityRoutePtr) -> Arc<DiagnosticReserve>;
}

pub trait DiagnosticQuery: DiagnosticSalsaQuery {
    fn print_diagnostics(&self) {
        let modules = self.all_modules();
        for module in modules.iter() {
            let diagnostic_reserve = self.diagnostics_reserve(*module);
            p!(module);
            p!(self.module_file(*module));
            p!(diagnostic_reserve.data());
        }
    }

    fn all_diagnostics(&self) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        for module in self.all_modules() {
            let diagnostics_reserve = self.diagnostics_reserve(module);
            diagnostics.extend(diagnostics_reserve.data().iter().map(|d| d.clone()));
        }
        diagnostics
    }
}

fn diagnostics_reserve(
    this: &dyn DiagnosticSalsaQuery,
    module: EntityRoutePtr,
) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_module_diagnostics(
        this, module,
    )))
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
