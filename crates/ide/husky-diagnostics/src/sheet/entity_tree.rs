use super::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct EntityTreeDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn entity_tree_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> EntityTreeDiagnosticSheet {
    let mut diagnostics = vec![];
    // todo
    EntityTreeDiagnosticSheet::new(db, diagnostics)
}
