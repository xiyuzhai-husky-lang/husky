use super::*;

#[salsa::tracked]
pub struct MainCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_main_crate_syn_node_decl(self) -> MainCrateSynNodeDecl {
        todo!()
    }
}

#[salsa::tracked]
pub struct MainCrateSynDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl MainCrateSynDecl {
    pub(super) fn from_node(
        path: CratePath,
        syn_node_decl: MainCrateSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, syn_expr_region))
    }
}
