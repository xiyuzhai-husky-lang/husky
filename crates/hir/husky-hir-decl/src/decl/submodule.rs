use super::*;

#[salsa::tracked(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct SubmoduleHirDecl {
    #[id]
    pub path: SubmodulePath,
}

impl HasHirDecl for SubmodulePath {
    type HirDecl = SubmoduleHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        Some(submodule_hir_decl(db, self))
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn submodule_hir_decl(db: &dyn HirDeclDb, path: SubmodulePath) -> SubmoduleHirDecl {
    SubmoduleHirDecl::new(db, path)
}
