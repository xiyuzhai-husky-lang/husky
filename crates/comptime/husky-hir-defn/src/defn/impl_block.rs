use super::*;

impl HasHirDefn for ImplBlockPath {
    type HirDefn = ImplBlockHirDecl;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        Ok(self.decl(db)?)
    }
}
