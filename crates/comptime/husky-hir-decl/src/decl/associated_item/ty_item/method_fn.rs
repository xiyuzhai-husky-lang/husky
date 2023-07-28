use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMethodFnHirDecl {
    pub path: TypeItemPath,
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    #[return_ref]
    pub self_parameter: EtherealTermRitchieRegularParameter,
    #[return_ref]
    pub parenic_parameters: ParenicEtherealParameters,
    pub return_ty: EtherealTerm,
    pub hir_expr_region: HirEagerExprRegion,
}
