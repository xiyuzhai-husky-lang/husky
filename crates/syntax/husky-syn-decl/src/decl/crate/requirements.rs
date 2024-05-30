use super::*;

#[salsa::tracked]
pub struct RequirementsCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_requirements_crate_syn_node_decl(self) -> RequirementsCrateSynNodeDecl {
        let db = self.db();
        let path = self.crate_path();
        let syn_expr_region = self.finish();
        RequirementsCrateSynNodeDecl::new(db, path, syn_expr_region)
    }
}

#[salsa::tracked]
pub struct RequirementsCrateSynDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl RequirementsCrateSynDecl {
    pub(super) fn from_node(
        path: CratePath,
        syn_node_decl: RequirementsCrateSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, syn_expr_region))
    }
}
