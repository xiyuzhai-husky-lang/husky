use super::*;

#[salsa::tracked(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct SubmoduleHirDecl {
    #[id]
    pub path: ModulePath,
}

impl HasHirDecl for ModulePath {
    type HirDecl = SubmoduleHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        Some(submodule_hir_decl(db, self))
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn submodule_hir_decl(db: &dyn HirDeclDb, path: ModulePath) -> SubmoduleHirDecl {
    SubmoduleHirDecl::new(db, path)
}
