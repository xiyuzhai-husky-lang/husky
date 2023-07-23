use super::*;

impl HasHirDefn for ImplBlockPath {
    type HirDefn = ImplBlockHirDecl;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        self.hir_decl(db)
    }
}
