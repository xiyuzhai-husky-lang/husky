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
    if let (Ok(ranged_token_sheet), Ok(defns)) =
        (db.ranged_token_sheet(module_path), module_path.defns(db))
    {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
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
            // Defn::Type(Defn) => match Defn {
            //     TypeDefn::Enum(_Defn) => (),
            //     TypeDefn::PropsStruct(_Defn) => (),
            //     TypeDefn::UnitStruct(_Defn) => (),
            //     TypeDefn::TupleStruct(_Defn) => (),
            //     TypeDefn::Record(_Defn) => (),
            //     TypeDefn::Inductive(_Defn) => (),
            //     TypeDefn::Structure(_Defn) => (),
            //     TypeDefn::Extern(_Defn) => (),
            //     TypeDefn::Union(_Defn) => (),
            // },
            // Defn::Fugitive(_Defn) => (),
            // Defn::Trait(_Defn) => (),
            SynDefn::ImplBlock(_Defn) => (),
            SynDefn::AssociatedItem(_Defn) => (),
            SynDefn::TypeVariant(_Defn) => (),
        }
    }
}
