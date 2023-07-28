use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitMethodFnHirDecl {
    pub path: TraitItemPath,
    pub template_parameters: EtherealTemplateParameters,
    /// `Self` as generic parameter
    pub self_ty_template_parameter: EtherealTemplateParameter,
    pub hir_expr_region: HirEagerExprRegion,
}
