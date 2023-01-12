use super::*;

#[salsa::tracked(jar = DiagnosticsJar)]
pub struct EntityTreeDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn entity_tree_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> EntityTreeDiagnosticSheet {
    todo!()
}
