use crate::*;

#[salsa::tracked(jar = DiagnosticsJar)]
pub struct DiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn diagnostic_sheet(db: &dyn DiagnosticsDb, module_path: ModulePath) -> DiagnosticSheet {
    todo!()
}
