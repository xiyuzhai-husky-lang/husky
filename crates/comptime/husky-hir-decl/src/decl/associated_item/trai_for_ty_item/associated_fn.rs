use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedFnHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub ty_term: EtherealTerm,
    pub hir_expr_region: HirEagerExprRegion,
}
