use super::*;

#[salsa::tracked]
pub struct LibCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_lib_crate_syn_node_decl(&self) -> LibCrateSynNodeDecl {
        let db = self.db();
        let path = self.crate_path();
        let parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        let syn_expr_region = parser.finish();
        LibCrateSynNodeDecl::new(db, path, syn_expr_region)
    }
}
