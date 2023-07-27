use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitHirDecl {
    pub path: TraitPath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub hir_expr_region: HirEagerExprRegion,
}
