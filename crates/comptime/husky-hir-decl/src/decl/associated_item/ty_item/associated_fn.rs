use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedFnHirDecl {
    pub path: TypeItemPath,
    pub self_ty: HirType,
    pub template_parameters: HirTemplateParameters,
    pub parenate_parameters: EtherealTermParenateParameters,
    pub return_ty: HirType,
    pub ty: HirType,
    pub hir_expr_region: HirEagerExprRegion,
}
