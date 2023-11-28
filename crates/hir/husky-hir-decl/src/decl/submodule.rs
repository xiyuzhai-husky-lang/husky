use super::*;

#[salsa::tracked(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct SubmoduleHirDecl {
    #[id]
    pub path: SubmoduleItemPath,
}

impl HasHirDecl for SubmoduleItemPath {
    type HirDecl = SubmoduleHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        Some(submodule_hir_decl(db, self))
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn submodule_hir_decl(db: &::salsa::Db, path: SubmoduleItemPath) -> SubmoduleHirDecl {
    SubmoduleHirDecl::new(db, path)
}
