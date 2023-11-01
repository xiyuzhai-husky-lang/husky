use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitAssociatedValHirDecl {
    pub path: TraitItemPath,
    pub hir_expr_region: HirEagerExprRegion,
}
