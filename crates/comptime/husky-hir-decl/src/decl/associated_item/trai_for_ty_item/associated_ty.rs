use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedTypeHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    pub ty_term: EtherealTerm,
    pub hir_expr_region: HirEagerExprRegion,
}
