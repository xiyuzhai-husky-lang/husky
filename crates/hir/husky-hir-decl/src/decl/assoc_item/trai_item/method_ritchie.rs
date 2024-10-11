use super::*;

#[salsa::interned]
pub struct TraitMethodFnHirDecl {
    pub path: TraitItemPath,
    pub template_parameters: HirTemplateParameters,
    /// `Self` as generic parameter
    pub self_ty_template_parameter: HirTemplateParameter,
    pub self_value_parameter: HirEagerSelfValueParameter,
    #[return_ref]
    pub parenate_parameters: HirEagerParenateParameters,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}
