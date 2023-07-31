use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct TraitHirDefn {
    pub path: TraitPath,
    pub hir_decl: TraitHirDecl,
}

impl HasHirDefn for TraitPath {
    type HirDefn = TraitHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        trai_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_hir_defn(db: &dyn HirDefnDb, path: TraitPath) -> Option<TraitHirDefn> {
    todo!()
    // let hir_decl = path.hir_decl(db)?;
    // (TraitHirDefn::new(db, path, hir_decl))
}
