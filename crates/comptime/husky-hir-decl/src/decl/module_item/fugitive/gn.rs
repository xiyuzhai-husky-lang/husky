use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct GnHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub hir_expr_region: HirExprRegion,
}
