use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImplBlockHirDefn {
    hir_decl: ImplBlockHirDecl,
}

impl ImplBlockHirDefn {
    pub fn hir_decl(self) -> ImplBlockHirDecl {
        self.hir_decl
    }
}

impl HasHirDefn for ImplBlockPath {
    type HirDefn = ImplBlockHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        Some(ImplBlockHirDefn {
            hir_decl: self.hir_decl(db)?,
        })
    }
}
