use super::*;
use husky_syn_defn::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DefnDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn defn_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> DefnDiagnosticSheet {
    let mut sheet_collector = ModuleDiagnosticsCollector::new(db, module_path);
    if let Ok(defns) = module_path.defns(db) {
        for defn in defns.iter().copied() {
            if let Some(syn_expr_region) = defn.syn_expr_region(db) {
                let mut region_collector =
                    RegionDiagnosticsCollector::new(db, syn_expr_region, &mut sheet_collector);
                region_collector.visit_defn(defn)
            }
        }
    }
    // todo
    DefnDiagnosticSheet::new(db, sheet_collector.finish())
}

impl<'a, 'b> RegionDiagnosticsCollector<'a, 'b> {
    fn visit_defn(&mut self, defn: SynDefn) {
        // todo
        match defn {
            SynDefn::Submodule(_) => (),
            SynDefn::MajorItem(_) => (),
            SynDefn::ImplBlock(_) => (),
            SynDefn::AssociatedItem(_) => (),
            SynDefn::TypeVariant(_) => (),
        }
    }
}
