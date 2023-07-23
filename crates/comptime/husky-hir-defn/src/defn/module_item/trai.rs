use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct TraitHirDefn {
    #[id]
    pub path: TraitPath,
    pub decl: TraitHirDecl,
}

impl HasHirDefn for TraitPath {
    type HirDefn = TraitHirDefn;

    fn syn_defn(self, db: &dyn HirDefnDb) -> HirDefnResult<Self::HirDefn> {
        trai_syn_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_syn_defn(db: &dyn HirDefnDb, path: TraitPath) -> HirDefnResult<TraitHirDefn> {
    let decl = path.decl(db)?;
    Ok(TraitHirDefn::new(db, path, decl))
}
