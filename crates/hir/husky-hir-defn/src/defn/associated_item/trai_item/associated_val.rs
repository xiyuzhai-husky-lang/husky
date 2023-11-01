use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedValHirDefn {
    pub syn_node_path: TraitItemPath,
    pub hir_decl: TraitAssociatedValHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}
