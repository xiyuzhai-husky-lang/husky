use husky_ast::AstQueryGroup;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::EntityDefnQueryGroup;
use husky_entity_syntax::EntitySyntaxQueryGroup;
use infer_total::InferQueryGroup;
use reserve::Reserve;

use crate::*;

#[salsa::query_group(DiagnosticQueryGroupStorage)]
pub trait DiagnosticQuery:
    EntitySyntaxQueryGroup + AstQueryGroup + InferQueryGroup + EntityDefnQueryGroup
{
    fn diagnostics_reserve(&self, module: EntityRoutePtr) -> Arc<DiagnosticReserve>;
}

fn diagnostics_reserve(
    this: &dyn DiagnosticQuery,
    module: EntityRoutePtr,
) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_diagnostics(this, module)))
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
