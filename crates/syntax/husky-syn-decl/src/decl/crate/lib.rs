use super::*;

#[salsa::tracked]
pub struct LibCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_lib_crate_syn_node_decl(&self) -> CrateSynNodeDecl {
        todo!()
    }
}
