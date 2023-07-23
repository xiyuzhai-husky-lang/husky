use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedTypeHirDecl {
    pub path: TypeItemPath,
    pub hir_expr_region: HirExprRegion,
}
