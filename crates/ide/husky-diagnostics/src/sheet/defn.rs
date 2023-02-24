use super::*;
use husky_defn::*;

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
    let mut sheet_collector = SheetDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(defn_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.collect_defns(module_path),
    ) {
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (path, defn) in defn_sheet.defns() {
            if let Ok(defn) = defn {
                if let Some(expr_region) = defn.expr_region(db) {
                    let mut region_collector =
                        RegionDiagnosticsCollector::new(db, expr_region, &mut sheet_collector);
                    region_collector.visit_defn(defn)
                }
            }
        }
    }
    // todo
    DefnDiagnosticSheet::new(db, sheet_collector.finish())
}

impl<'a, 'b> RegionDiagnosticsCollector<'a, 'b> {
    fn visit_defn(&mut self, defn: Defn) {
        match defn {
            Defn::Type(Defn) => match Defn {
                TypeDefn::Enum(Defn) => (),
                TypeDefn::RegularStruct(Defn) => (),
                TypeDefn::UnitStruct(Defn) => (),
                TypeDefn::TupleStruct(Defn) => (),
                TypeDefn::Record(Defn) => (),
                TypeDefn::Inductive(Defn) => (),
                TypeDefn::Structure(Defn) => (),
                TypeDefn::Alien(Defn) => (),
                TypeDefn::Union(Defn) => (),
            },
            Defn::Form(Defn) => (),
            Defn::Trait(Defn) => (),
            Defn::Impl(Defn) => (),
            Defn::AssociatedItem(Defn) => (),
            Defn::Variant(Defn) => (),
        }
    }
}
