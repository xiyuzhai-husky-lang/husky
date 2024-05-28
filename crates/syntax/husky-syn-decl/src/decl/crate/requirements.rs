use super::*;

#[salsa::tracked]
pub struct RequirementsCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_requirements_crate_syn_node_decl(&self) -> RequirementsCrateSynNodeDecl {
        todo!()
    }
}
