use reserve::Reserve;
use scope_query::{PackageOrModule, ScopeQueryGroup};

use crate::*;

#[salsa::query_group(DiagnosticQueryStorage)]
pub trait DiagnosticQuery: ScopeQueryGroup {
    fn diagnostic_reserve(&self, module: PackageOrModule) -> Arc<DiagnosticReserve>;
}

fn diagnostic_reserve(
    this: &dyn DiagnosticQuery,
    module: PackageOrModule,
) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_diagnostics(this, module)))
}

pub type DiagnosticReserve = Reserve<Vec<Diagnostic>>;
