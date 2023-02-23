use super::*;
use husky_decl::{Decl, DeclResult, DeclResultRef};

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DeclDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn decl_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> DeclDiagnosticSheet {
    let mut center = RegionDiagnosticsCollectorCenter::default();
    if let (Ok(ranged_token_sheet), Ok(decl_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.decl_sheet(module_path),
    ) {
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (path, decl) in decl_sheet.decls().iter().copied() {
            match decl {
                Ok(decl) => {
                    let mut collector =
                        RegionDiagnosticsCollector::new(db, decl.expr_region(db), &mut center);
                    collector.visit_decl(decl)
                }
                Err(_) => todo!(),
            }
        }
    }
    // todo
    DeclDiagnosticSheet::new(db, center.finish())
}

impl<'a> RegionDiagnosticsCollector<'a> {
    fn visit_decl(&mut self, decl: Decl) {
        todo!()
    }
}
