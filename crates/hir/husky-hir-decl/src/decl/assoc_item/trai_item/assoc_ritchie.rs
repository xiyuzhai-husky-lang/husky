use super::*;

#[salsa::interned]
pub struct TraitAssocRitchieHirDecl {
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub parenate_parameters: HirEagerParenateParameters,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}
