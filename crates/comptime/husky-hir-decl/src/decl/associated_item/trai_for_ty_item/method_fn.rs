use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeMethodFnHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub self_parameter: EtherealTermRitchieRegularParameter,
    #[return_ref]
    pub parenic_parameters: ParenicEtherealParameters,
    pub return_ty: EtherealTerm,
    pub hir_expr_region: HirExprRegion,
}
