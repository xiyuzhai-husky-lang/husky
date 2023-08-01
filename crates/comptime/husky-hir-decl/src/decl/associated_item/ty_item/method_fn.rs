use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMethodFnHirDecl {
    pub path: TypeItemPath,
    pub self_ty: HirType,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub self_parameter: EtherealTermRitchieRegularParameter,
    #[return_ref]
    pub parenate_parameters: EtherealTermParenateParameters,
    pub return_ty: HirType,
    pub hir_expr_region: HirEagerExprRegion,
}
