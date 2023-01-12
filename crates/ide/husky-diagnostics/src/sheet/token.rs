use super::*;

#[salsa::tracked(jar = DiagnosticsJar)]
pub struct TokenDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn token_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> TokenDiagnosticSheet {
    todo!()
}
