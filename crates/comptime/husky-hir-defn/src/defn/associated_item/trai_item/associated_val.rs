use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedValHirDefn {
    #[id]
    pub syn_node_path: TraitItemPath,
    pub hir_decl: TraitAssociatedValHirDecl,
    pub hir_expr_region: HirExprRegion,
}
