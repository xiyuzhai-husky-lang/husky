use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedValHirDefn {
    #[id]
    pub syn_node_path: TraitItemPath,
    pub decl: TraitAssociatedValHirDecl,
    pub expr_region: HirExprRegion,
}
