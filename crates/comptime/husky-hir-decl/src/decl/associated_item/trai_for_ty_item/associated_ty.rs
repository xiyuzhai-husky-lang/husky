use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedTypeHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub ty_term: EtherealTerm,
    pub hir_expr_region: HirExprRegion,
}
