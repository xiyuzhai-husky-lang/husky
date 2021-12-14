use crate::*;

#[salsa::query_group(DiagnosticQueryStorage)]
pub trait DiagnosticQuery {
    fn diagnostic_reserve(&self, module: scope::Module) -> Arc<DiagnosticReserve>;
}

fn diagnostic_reserve(this: &dyn DiagnosticQuery, module: scope::Module) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_diagnostics(this, module)))
}
