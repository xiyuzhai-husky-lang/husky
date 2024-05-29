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
