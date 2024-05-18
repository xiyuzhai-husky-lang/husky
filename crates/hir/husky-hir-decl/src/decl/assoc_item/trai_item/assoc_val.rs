use super::*;

#[salsa::interned]
pub struct TraitAssocValHirDecl {
    pub path: TraitItemPath,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}
