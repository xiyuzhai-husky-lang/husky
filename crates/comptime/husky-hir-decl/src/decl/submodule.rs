use super::*;

#[salsa::tracked(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct SubmoduleHirDecl {
    #[id]
    pub syn_node_path: ModulePath,
}

impl HasHirDecl for ModulePath {
    type HirDecl = SubmoduleHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        todo!()
    }
}
