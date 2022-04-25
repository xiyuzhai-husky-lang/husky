use ast::AstQueryGroup;
use entity_route::EntityRoutePtr;
use entity_route_query::EntityRouteQueryGroup;
use infer_total::InferQueryGroup;
use reserve::Reserve;

use crate::*;

#[salsa::query_group(DiagnosticQueryGroupStorage)]
pub trait DiagnosticQuery: EntityRouteQueryGroup + AstQueryGroup + InferQueryGroup {
    fn diagnostic_reserve(&self, module: EntityRoutePtr) -> Arc<DiagnosticReserve>;
}

fn diagnostic_reserve(
    this: &dyn DiagnosticQuery,
    module: EntityRoutePtr,
) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_diagnostics(this, module)))
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
