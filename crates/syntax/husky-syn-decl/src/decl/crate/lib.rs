use super::*;

#[salsa::tracked]
pub struct LibCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_lib_crate_syn_node_decl(self) -> LibCrateSynNodeDecl {
        let db = self.db();
        let path = self.crate_path();
        let syn_expr_region = self.finish();
        LibCrateSynNodeDecl::new(db, path, syn_expr_region)
    }
}
