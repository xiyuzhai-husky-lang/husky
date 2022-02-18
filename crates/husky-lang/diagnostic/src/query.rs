use reserve::Reserve;
use scope::ScopePtr;
use scope_query::ScopeQueryGroup;

use crate::*;

#[salsa::query_group(DiagnosticQueryStorage)]
pub trait DiagnosticQuery: ScopeQueryGroup {
    fn diagnostic_reserve(&self, module: ScopePtr) -> Arc<DiagnosticReserve>;
}

fn diagnostic_reserve(this: &dyn DiagnosticQuery, module: ScopePtr) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_diagnostics(this, module)))
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
