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
