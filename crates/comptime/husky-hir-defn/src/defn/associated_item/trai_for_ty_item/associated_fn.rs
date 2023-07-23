use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnHirDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssociatedFnHirDecl,
    pub hir_expr_region: HirExprRegion,
}
