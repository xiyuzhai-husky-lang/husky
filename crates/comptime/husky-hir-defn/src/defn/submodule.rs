use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = HirDefnDb, jar = HirDefnJar)]
pub struct SubmoduleHirDefn {
    decl: SubmoduleHirDecl,
}

impl SubmoduleHirDefn {
    pub fn decl(self) -> SubmoduleHirDecl {
        self.decl
    }
}

impl HasHirDefn for ModulePath {
    type HirDefn = SubmoduleHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        SubmoduleHirDefn {
            decl: self.hir_decl(db),
        }
    }
}
