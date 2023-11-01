use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssociatedFnHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}
