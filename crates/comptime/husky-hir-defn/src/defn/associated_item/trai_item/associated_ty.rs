use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeHirDefn {
    #[id]
    pub path: TraitItemPath,
    pub hir_decl: TraitAssociatedTypeHirDecl,
    pub hir_expr_region: HirExprRegion,
}
