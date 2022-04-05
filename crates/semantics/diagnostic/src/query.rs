use entity_route::EntityRoutePtr;
use entity_route_query::ScopeQueryGroup;
use reserve::Reserve;

use crate::*;

#[salsa::query_group(DiagnosticQueryStorage)]
pub trait DiagnosticQuery: ScopeQueryGroup {
    fn diagnostic_reserve(&self, module: EntityRoutePtr) -> Arc<DiagnosticReserve>;
}

fn diagnostic_reserve(
    this: &dyn DiagnosticQuery,
    module: EntityRoutePtr,
) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_diagnostics(this, module)))
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
